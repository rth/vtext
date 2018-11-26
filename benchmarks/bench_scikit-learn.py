from time import time
from glob import glob

from sklearn.feature_extraction.text import HashingVectorizer
from sklearn.feature_extraction.text import CountVectorizer

if __name__ == '__main__':
    input_files = list(glob('./data/*/*'))
    data = []
    for file_path in input_files:
        with open(file_path, 'rt') as fh:
            data.append(fh.read())

    t0 = time()
    vect = CountVectorizer(lowercase=False)
    vect.fit_transform(data)

    dt = time() - t0

    print(f"CountVectorizer: vectorized {len(data)} documents in {dt:.2f}s")

    t0 = time()
    vect = HashingVectorizer(lowercase=False)
    vect.fit_transform(data)

    dt = time() - t0

    print(f"HashingVectorizer: vectorized {len(data)} documents in {dt:.2f}s")
