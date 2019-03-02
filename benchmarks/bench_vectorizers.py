from time import time
from glob import glob

import sklearn.feature_extraction.text as skt

import pytext_vectorize


if __name__ == "__main__":
    input_files = list(glob("./data/*/*"))
    data = []
    for file_path in input_files:
        with open(file_path, "rt") as fh:
            data.append(fh.read())

    dataset_size = 91  # MB for 20 newsgroup dataset

    print("# vectorizing {} documents:".format(len(data)))

    for label, vect in [
        (
            "HashingVectorizer (text-vectorize)",
            pytext_vectorize.HashingVectorizer(norm=None),
        ),
        (
            "HashingVectorizer (scikit-learn)",
            skt.HashingVectorizer(lowercase=False, norm=None),
        ),
        (
            "CountVectorizer (text-vectorize)",
            pytext_vectorize.CountVectorizer(lowercase=False),
        ),
        ("CountVectorizer (scikit-learn)", skt.CountVectorizer(lowercase=False)),
    ]:

        t0 = time()

        X = vect.fit_transform(data)

        dt = time() - t0

        print(
            "{:>40}: {:.2f}s [{:.1f} MB/s], shape={}, nnz={}".format(
                label, dt, dataset_size / dt, X.shape, X.nnz
            )
        )
