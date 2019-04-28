.. vtext documentation master file, created by
   sphinx-quickstart on Sun Apr 14 23:28:11 2019.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

vtext
=====

NLP in Rust with Python bindings

This package aims to provide a high performance toolkit for ingesting textual data for
machine learning applications.

The API is currently unstable.

Features
--------

 - Tokenization: Regexp tokenizer, Unicode segmentation
 - Stemming: Snowball (in Python 15-20x faster than NLTK) + language specific rules
 - Analyzers (*planned*): word and character n-grams, skip grams
 - Token counting: converting token counts to sparse matrices for use
   in machine learning libraries. Similar to `CountVectorizer` and
   `HashingVectorizer` in scikit-learn.
 - Feature weighting (*planned*): feature weighting based on document
   frequency (TF-IDF), feature normalization.

.. toctree::
   :maxdepth: 2
   :caption: Contents

   install
   user-manual
   benchmarks
   rust-api
   python-api
   contributing



