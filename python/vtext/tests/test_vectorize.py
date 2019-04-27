import pickle

from numpy.testing import assert_array_equal

import pytest

from vtext.vectorize import HashingVectorizer, CountVectorizer


def test_count_vectorizer():
    text = ["some sentence", "a different sentence"]
    vect = CountVectorizer()
    vect.fit(text)
    X2 = vect.transform(text)

    vect = CountVectorizer()
    X = vect.fit_transform(text)
    assert X.nnz == 4
    assert_array_equal(X.indices, X2.indices)


def test_hashing_vectorizer():
    text = ["some sentence", "a different sentence"]
    vect = HashingVectorizer(norm=None)
    vect.fit(text)
    X2 = vect.transform(text)

    vect = HashingVectorizer(norm=None)
    X = vect.fit_transform(text)
    assert X.nnz == 4
    assert_array_equal(X.indices, X2.indices)


@pytest.mark.parametrize("Estimator", [HashingVectorizer])
def test_pickle_vectorizers(Estimator):

    vect = Estimator()

    out = pickle.dumps(vect)

    pickle.loads(out)
