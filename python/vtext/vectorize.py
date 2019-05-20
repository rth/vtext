# Copyright 2019 vtext developers
#
# Licensed under the Apache License, Version 2.0,
# <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
# modified, or distributed except according to those terms.

import numpy as np

from . import _lib
import scipy.sparse as sp

try:
    from sklearn.base import BaseEstimator
except ImportError:

    class BaseEstimator:
        pass


__all__ = ["CountVectorizer", "HashingVectorizer"]


class HashingVectorizer(BaseEstimator):
    """Convert a collection of text documents to a matrix of token occurrences

    It turns a collection of text documents into a scipy.sparse matrix holding
    token occurrence counts, similarly to the scikit-learn estimator with the
    same name.

    This text vectorizer implementation uses the hashing trick to find the
    token string name to feature integer index mapping.

    .. note::
       Experimental more parameters will be added in the future


    Parameters
    ----------

    analyzer : string, {'word'}
        Whether the feature should be made of word or character n-grams.

        Currently only the "word" tokenizer is implemented.
    binary : boolean, default=False
        If True, all non zero counts are set to 1. This is useful for discrete
        probabilistic models that model binary events rather than integer
        counts.
    n_jobs : int, default=1
        number of threads to use for parallel feature extraction. n_jobs > 1,
        is faster, but uses more memory.

        Note: currently any value n_jobs > 1 will use all available cores.
    dtype : type, optional
        Type of the matrix returned by fit_transform() or transform().


    Examples
    --------
    >>> from vtext.vectorize import HashingVectorizer
    >>> corpus = [
    ...     'This is the first document.',
    ...     'This document is the second document.',
    ...     'And this is the third one.',
    ...     'Is this the first document?',
    ... ]
    >>> vectorizer = HashingVectorizer()
    >>> X = vectorizer.fit_transform(corpus)
    >>> print(X.shape)
    (4, 16)

    """

    def __init__(self, *, analyzer="word", binary=False, n_jobs=1, dtype=np.int32):
        self.analyzer = analyzer
        self.binary = binary
        self.dtype = dtype
        self.n_jobs = n_jobs

    def partial_fit(self, X, y=None):
        """Does nothing: this transformer is stateless.

        This method is just there to mark the fact that this transformer
        can work in a streaming setup.

        Parameters
        ----------
        X : array-like, shape [n_samples, n_features]
            Training data.
        """
        return self

    def _validate_params(self):

        if self.analyzer != "word":
            raise NotImplementedError

        if not isinstance(self.n_jobs, int) or self.n_jobs < 1:
            raise ValueError("n_jobs={} must be a integer >= 1".format(self.n_jobs))

    def fit(self, X, y=None):
        """Does nothing: this transformer is stateless.

        Parameters
        ----------
        X : array-like, shape [n_samples, n_features]
            Training data.
        """
        if isinstance(X, str):
            raise ValueError(
                "Iterable over raw text documents expected, string object received."
            )

        self._validate_params()
        self._inner = _lib._HashingVectorizerWrapper(n_jobs=self.n_jobs)

        return self

    def transform(self, X):
        """Transform a sequence of documents to a document-term matrix.

        Parameters
        ----------
        X : iterable over raw text documents, length = n_samples
            Samples. Each sample must be a text document (either bytes or
            unicode strings, file name or file object depending on the
            constructor argument) which will be tokenized and hashed.

        Returns
        -------
        X : scipy.sparse matrix, shape = (n_samples, self.n_features)
            Document-term matrix.
        """
        if isinstance(X, str):
            raise ValueError(
                "Iterable over raw text documents expected, string object received."
            )

        self._validate_params()

        if not hasattr(self, "_inner"):
            # Initialize the wrapper
            self.fit(None)

        indices, indptr, data = self._inner.transform(X)
        if self.binary:
            data.fill(1)

        data = data.astype(self.dtype, copy=False)
        return sp.csr_matrix((data, indices, indptr), shape=(len(X), 1048576))

    def fit_transform(self, X, y=None):
        """Transform a sequence of documents to a document-term matrix.

        Parameters
        ----------
        X : iterable over raw text documents, length = n_samples
            Samples. Each sample must be a text document (either bytes or
            unicode strings, file name or file object depending on the
            constructor argument) which will be tokenized and hashed.
        y : any
            Ignored. This parameter exists only for compatibility with
            sklearn.pipeline.Pipeline.

        Returns
        -------
        X : scipy.sparse matrix, shape = (n_samples, self.n_features)
            Document-term matrix.
        """
        return self.fit(X, y).transform(X)


class CountVectorizer(BaseEstimator):
    """Convert a collection of text documents to a matrix of token counts

    This implementation produces a sparse representation of the counts using
    scipy.sparse.csr_matrix.

    If you do not provide an a-priori dictionary and you do not use an analyzer
    that does some kind of feature selection then the number of features will
    be equal to the vocabulary size found by analyzing the data.

    Parameters
    ----------
    analyzer : string, {'word'}
        Whether the feature should be made of word or character n-grams.

        Currently only `analyzer="word"` is implemented.

    binary : boolean, default=False
        If True, all non zero counts are set to 1. This is useful for discrete
        probabilistic models that model binary events rather than integer
        counts.
    n_jobs : int, default=1
        number of threads to use for parallel feature extraction. n_jobs > 1,
        is faster, but uses more memory.

        Note: currently any value n_jobs > 1 will use all available cores.

    dtype : type, optional
        Type of the matrix returned by fit_transform() or transform().

    Attributes
    ----------
    vocabulary_ : dict
        A mapping of terms to feature indices.


    Examples
    --------
    >>> from vtext.tokenize import CountVectorizer
    >>> corpus = [
    ...     'This is the first document.',
    ...     'This document is the second document.',
    ...     'And this is the third one.',
    ...     'Is this the first document?',
    ... ]
    >>> vectorizer = CountVectorizer()
    >>> X = vectorizer.fit_transform(corpus)
    >>> print(vectorizer.get_feature_names())
    ['and', 'document', 'first', 'is', 'one', 'second', 'the', 'third', 'this']
    >>> print(X.toarray())  # doctest: +NORMALIZE_WHITESPACE
    [[0 1 1 1 0 0 1 0 1]
     [0 2 0 1 0 1 1 0 1]
     [1 0 0 1 1 0 1 1 1]
     [0 1 1 1 0 0 1 0 1]]

    See also
    --------
    HashingVectorizer

    """

    def __init__(self, *, analyzer="word", binary=False, n_jobs=1, dtype=np.int64):
        self.analyzer = analyzer
        self.binary = binary
        self.n_jobs = n_jobs
        self.dtype = dtype

    def _check_vocabulary(self):
        pass

    def _validate_vocabulary(self):
        pass

    def _validate_params(self):

        if self.analyzer != "word":
            raise NotImplementedError

        if not isinstance(self.n_jobs, int) or self.n_jobs < 1:
            raise ValueError("n_jobs={} must be a integer >= 1".format(self.n_jobs))

    def fit(self, raw_documents, y=None):
        """Learn a vocabulary dictionary of all tokens in the raw documents.

        Parameters
        ----------
        raw_documents : iterable
            An iterable which yields either str, unicode or file objects.

        Returns
        -------
        self
        """
        self._validate_params()
        self._vect = _lib._CountVectorizerWrapper(self.n_jobs)
        self._vect.fit(raw_documents)
        return self

    def fit_transform(self, raw_documents, y=None):
        """Learn the vocabulary dictionary and return term-document matrix.

        This is equivalent to fit followed by transform, but more efficiently
        implemented.

        Parameters
        ----------
        raw_documents : iterable
            An iterable which yields either str, unicode or file objects.

        Returns
        -------
        X : array, [n_samples, n_features]
            Document-term matrix.
        """
        # We intentionally don't call the transform method to make
        # fit_transform overridable without unwanted side effects in
        # TfidfVectorizer.
        if isinstance(raw_documents, str):
            raise ValueError(
                "Iterable over raw text documents expected, " "string object received."
            )

        self._validate_params()
        self._validate_vocabulary()

        self._vect = _lib._CountVectorizerWrapper(n_jobs=self.n_jobs)
        indices, indptr, data = self._vect.fit_transform(raw_documents)
        n_features = self._vect.get_n_features()
        X = sp.csr_matrix((data, indices, indptr), shape=(len(indptr) - 1, n_features))

        if self.binary:
            X.data.fill(1)

        return X

    def transform(self, raw_documents):
        """Transform documents to document-term matrix.

        Extract token counts out of raw text documents using the vocabulary
        fitted with fit or the one provided to the constructor.

        Parameters
        ----------
        raw_documents : iterable
            An iterable which yields either str, unicode or file objects.

        Returns
        -------
        X : sparse matrix, [n_samples, n_features]
            Document-term matrix.
        """
        if isinstance(raw_documents, str):
            raise ValueError(
                "Iterable over raw text documents expected, string object received."
            )
        if not hasattr(self, "_vect"):
            raise ValueError("Model need to be fitted first!")

        if not hasattr(self, "vocabulary_"):
            self._validate_vocabulary()

        self._check_vocabulary()

        # use the same matrix-building strategy as fit_transform
        indices, indptr, data = self._vect.transform(raw_documents)
        n_features = self._vect.get_n_features()
        X = sp.csr_matrix((data, indices, indptr), shape=(len(indptr) - 1, n_features))

        if self.binary:
            X.data.fill(1)
        return X

    def inverse_transform(self, X):
        """Return terms per document with nonzero entries in X.

        .. note:: Not implemented

        Parameters
        ----------
        X : {array, sparse matrix}, shape = [n_samples, n_features]

        Returns
        -------
        X_inv : list of arrays, len = n_samples
            List of arrays of terms.
        """
        raise NotImplementedError()

    def get_feature_names(self):
        """Array mapping from feature integer indices to feature name

        .. note:: Not implemented
        """
        raise NotImplementedError()
