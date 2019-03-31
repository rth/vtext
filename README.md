# vtext

[![CircleCI](https://circleci.com/gh/rth/vtext/tree/master.svg?style=svg)](https://circleci.com/gh/rth/vtext/tree/master)

Text vectorizers and TFIDF transforms for Rust and Python

This package aims to provide a convenient and high performance toolkit for ingesting textual data for
machine learning applications.

The API is currently unstable.

## Features

**Tokenization**

 - Regexp tokenizer
 - Unicode segmentation tokenizer

**Stemming**

 - Snowball Stemmer: in Python 15-20x faster than NLTK

**Analyzers**

 - *Planned*: word and character n-grams, skip grams

**Token counting**

These estimators take as input a list of tokens, and convert it to a sparse CSR
array that can be used in machine learning applications.

 - `CountVectorizer` 
 - `HashingVectorizer` using MurmurHash3

API and implementation inspired by `CountVectorizer` and `HashingVectorizer`
estimators in [scikit-learn](https://scikit-learn.org/).

**Feature selection**

 - *Planned:*  filtering vocabulary by document frequency (`df`)

**Feature weighting**

 - *Planned:*  feature weighting based on document frequency (TF-IDF),
   supervised term weighting (e.g. TF-IGM), feature normalization

## Usage

### Usage in Rust

Add the following to `Cargo.toml`,
```toml
[dependencies]
text-vectorize = {"git" = "https://github.com/rth/vtext"}
``` 
A simple example can be found below,
```rust
extern crate vtext;

use vtext::CountVectorizer;

let documents = vec![
    String::from("Some text input"),
    String::from("Another line"),
];

let mut vect = CountVectorizer::new();
let X = vect.fit_transform(&documents);
```
where `X` is a `CSRArray` struct with the following attributes
`X.indptr`, `X.indices`, `X.values`.

### Usage in Python

Not implemented yet, see [#1](https://github.com/rth/text-vectorize/pull/1).

The API aims to be compatible with scikit-learn's
[CountVectorizer](https://scikit-learn.org/stable/modules/generated/sklearn.feature_extraction.text.CountVectorizer.html)
and [HashingVectorizer](https://scikit-learn.org/stable/modules/generated/sklearn.feature_extraction.text.HashingVectorizer.html) 
though only a subset of features will be implemented.


## Benchmarks

Below are some very preliminary benchmarks on the 20 newsgroups dataset of 19924 documents (~91 MB in total),

| estimator         | implementation                    | speed        |
|-------------------|-----------------------------------|--------------|
| CountVectorizer   | scikit-learn 0.20 (Python)        | 14 MB/s      |
| CountVectorizer   | text-vectorize 0.1.0-alpha (Rust) | 33 MB/s      |
| HashingVectorizer | scikit-learn 0.20 (Python+Cython) | 18 MB/s      |
| HashingVectorizer | text-vectorize 0.1.0-alpha (Rust) | 68 MB/s      |

see [benchmarks/README.md](./benchmarks/README.md) for more details.
Note that these are not strictly equivalent, because
they do not account for cost of passing data from Rust to Python. Instead they are meant a
a rough estimate for the possible performance improvements.


## License

text-vectorize is released under the BSD 3-clause license.
