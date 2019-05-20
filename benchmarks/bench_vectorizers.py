from time import time
from glob import glob
from pathlib import Path

import sklearn.feature_extraction.text as skt

import vtext.vectorize

base_dir = Path(__file__).parent.parent.resolve()

if __name__ == "__main__":
    input_files = list(glob(str(base_dir / "data" / "*" / "*")))
    data = []
    for file_path in input_files:
        with open(file_path, "rt") as fh:
            data.append(fh.read())

    dataset_size = 91  # MB for 20 newsgroup dataset

    print("# vectorizing {} documents:".format(len(data)))

    for label, vect, method in [
        (
            "HashingVectorizer(n_jobs=1).transform [vtext]",
            vtext.vectorize.HashingVectorizer(),
            "fit_transform",
        ),
        (
            "HashingVectorizer(n_jobs=4).transform [vtext]",
            vtext.vectorize.HashingVectorizer(n_jobs=4),
            "fit_transform",
        ),
        (
            "HashingVectorizer().transform [scikit-learn]",
            skt.HashingVectorizer(lowercase=False, norm=None),
            "fit_transform",
        ),
        (
            "CountVectorizer(n_jobs=1).fit [vtext]",
            vtext.vectorize.CountVectorizer(),
            "fit",
        ),
        (
            "CountVectorizer(n_jobs=4).fit [vtext]",
            vtext.vectorize.CountVectorizer(n_jobs=4),
            "fit",
        ),
        (
            "CountVectorizer(n_jobs=1).transform [vtext]",
            vtext.vectorize.CountVectorizer().fit(data),
            "transform",
        ),
        (
            "CountVectorizer(n_jobs=4).transform [vtext]",
            vtext.vectorize.CountVectorizer(n_jobs=4).fit(data),
            "transform",
        ),
        (
            "CountVectorizer().fit_transform [vtext]",
            vtext.vectorize.CountVectorizer(),
            "fit_transform",
        ),
        (
            "CountVectorizer().fit_transform [scikit-learn]",
            skt.CountVectorizer(lowercase=True),
            "fit_transform",
        ),
        # (
        #     "CountVectorizer, 10-char ngram [scikit-learn]",
        #     skt.CountVectorizer(lowercase=True, analyzer="char", ngram_range=(10, 10)),
        #     "fit_transform"
        # ),
    ]:

        t0 = time()

        X = getattr(vect, method)(data)
        if not hasattr(X, "shape"):

            class X:
                shape = None
                nnz = None

        dt = time() - t0

        print(
            "{:>50}: {:.2f}s [{:.1f} MB/s], shape={}, nnz={}".format(
                label, dt, dataset_size / dt, X.shape, X.nnz
            )
        )
