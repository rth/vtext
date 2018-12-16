import pickle

import pytest
from text_vectorize import HashingVectorizer


def test_hashing_vectorizer():
    text = ["some sentence", "a different sentence"]
    vect = HashingVectorizer(norm=None)

    X = vect.fit_transform(text)
    assert X.nnz == 4


@pytest.mark.parametrize("Estimator", [HashingVectorizer])
def test_pickle_vectorizers(Estimator):

    vect = Estimator()

    out = pickle.dumps(vect)

    pickle.loads(out)
