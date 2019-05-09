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
            "HashingVectorizer (vtext, n_jobs=1)",
            vtext.vectorize.HashingVectorizer(),
            "fit_transform",
        ),
        (
            "HashingVectorizer (vtext, n_jobs=2)",
            vtext.vectorize.HashingVectorizer(n_jobs=2),
            "fit_transform",
        ),
        #(
        #    "HashingVectorizer (scikit-learn)",
        #    skt.HashingVectorizer(lowercase=False, norm=None),
        #    "fit_transform",
        #),
        ("CountVectorizer.fit (vtext)", vtext.vectorize.CountVectorizer(), "fit"),
        ("CountVectorizer.transform (vtext)", vtext.vectorize.CountVectorizer().fit(data), "transform"),
        ("CountVectorizer (vtext)", vtext.vectorize.CountVectorizer(), "fit_transform"),
        # (
        #     "CountVectorizer (scikit-learn)",
        #     skt.CountVectorizer(lowercase=False),
        #     "fit_transform",
        # ),
        # (
        #    "CountVectorizer, 4-char ngram (scikit-learn)",
        #    skt.CountVectorizer(lowercase=False, analyzer="char", ngram_range=(4, 4)),
        # ),
    ]:

        t0 = time()

        X = getattr(vect, method)(data)
        if not hasattr(X, "shape"):
            class X():
                shape = None
                nnz = None

        dt = time() - t0

        print(
            "{:>40}: {:.2f}s [{:.1f} MB/s], shape={}, nnz={}".format(
                label, dt, dataset_size / dt, X.shape, X.nnz
            )
        )
