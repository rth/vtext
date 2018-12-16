from time import time
from glob import glob
from scipy.sparse import csr_matrix

from sklearn.feature_extraction.text import HashingVectorizer
from sklearn.feature_extraction.text import CountVectorizer
from sklearn.feature_extraction.text import TfidfVectorizer

from text_vectorize._lib import hash_vectorize

if __name__ == "__main__":
    input_files = list(glob("./data/*/*"))
    data = []
    for file_path in input_files:
        with open(file_path, "rt") as fh:
            data.append(fh.read())

    dataset_size = 91  # MB for 20 newsgroup dataset

    t0 = time()
    indices, indptr, data_out = hash_vectorize(data)
    csr_matrix((data_out, indices, indptr))

    dt = time() - t0

    print(
        "HashingVectorizer (text-vectorize): vectorized {} "
        "documents in {:.2f}s [{:.1f} MB/s]".format(len(data), dt, dataset_size / dt)
    )

    t0 = time()
    vect = HashingVectorizer(lowercase=False, norm=None)
    vect.fit_transform(data)

    dt = time() - t0

    print(
        "HashingVectorizer (scikit-learn): vectorized {} "
        "documents in {:.2f}s [{:.1f} MB/s]".format(len(data), dt, dataset_size / dt)
    )

    t0 = time()
    vect = CountVectorizer(lowercase=False)
    vect.fit_transform(data)

    dt = time() - t0

    print(
        "CountVectorizer (scikit-learn): vectorized {} "
        "documents in {:.2f}s [{:.1f} MB/s]".format(len(data), dt, dataset_size / dt)
    )
    t0 = time()
    vect = TfidfVectorizer(lowercase=False, norm=None)
    vect.fit_transform(data)

    dt = time() - t0

    print(
        "TfidfVectorizer (scikit-learn): vectorized {} "
        "documents in {:.2f}s [{:.1f} MB/s]".format(len(data), dt, dataset_size / dt)
    )
