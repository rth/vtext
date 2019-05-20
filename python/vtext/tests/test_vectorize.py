# Copyright 2019 vtext developers
#
# Licensed under the Apache License, Version 2.0,
# <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
# modified, or distributed except according to those terms.

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
    vect = HashingVectorizer()
    vect.fit(text)
    X2 = vect.transform(text)

    vect = HashingVectorizer()
    X = vect.fit_transform(text)
    assert X.nnz == 4
    assert_array_equal(X.indices, X2.indices)


@pytest.mark.parametrize("Estimator", [HashingVectorizer])
def test_pickle_vectorizers(Estimator):

    vect = Estimator()

    out = pickle.dumps(vect)

    pickle.loads(out)


@pytest.mark.parametrize("Estimator", [HashingVectorizer, CountVectorizer])
def test_vectorizers_n_jobs(Estimator):
    """Check that parallel feature ingestion works"""
    text = ["Εν οίδα ότι ουδέν οίδα"]

    vect = Estimator(n_jobs=2)
    vect.fit(text)
    vect.transform(text)

    with pytest.raises(ValueError, match="n_jobs=0 must be a integer >= 1"):
        Estimator(n_jobs=0).fit(text)
