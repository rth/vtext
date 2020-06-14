.. vtext documentation master file, created by
   sphinx-quickstart on Sun Apr 14 23:28:11 2019.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

vtext
=====

NLP in Rust with Python bindings

This package aims to provide a high performance toolkit for ingesting textual data for
machine learning applications.

Features
--------

 - Tokenization: Regexp tokenizer, Unicode segmentation + language specific rules
 - Stemming: Snowball (in Python 15-20x faster than NLTK)
 - Token counting: converting token counts to sparse matrices for use
   in machine learning libraries. Similar to `CountVectorizer` and
   `HashingVectorizer` in scikit-learn.
 - Levenshtein edit distance; Sørensen-Dice, Jaro, Jaro Winkler string similarities

.. toctree::
   :maxdepth: 2
   :caption: Contents

   install
   benchmarks
   rust-api
   python-api
   contributing



