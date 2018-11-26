# text-vectorizers

Text vectorizers and TFIDF transforms in Rust with Python bindings (experimental)

Work in progress, the API is unstable.

API and implementation inspired by `CountVectorizer` and `HashingVectorizer`
estimators in [scikit-learn](https://scikit-learn.org/).

## Features

### Implemented

 - bag of word vectorization of text documents
 - hashing vectorizer using MurmurHash3

### Planned

 - Python wrapper ([#1](https://github.com/rth/text-vectorize/pull/1))
 - Support for word and character n-grams ([#2](https://github.com/rth/text-vectorize/issues/2))
 - Binary Python wheels ([#3](https://github.com/rth/text-vectorize/issues/3<Paste>))
 - IDF transforms and TfidfVectorizer ([#4](https://github.com/rth/text-vectorize/issues/4))

In general, see https://github.com/rth/text-vectorize/issues. Comments and suggestions are very welcome.


## Usage

### Usage in Rust

Add the following to `Cargo.toml`,
```
[dependencies]
text-vectorize = {"git" = "https://github.com/rth/text-vectorize"}
``` 

```
external crate text_vectorize;

use text_vectorize::CountVectorizer;

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

The API aims to be identical to scikit-learn's
[CountVectorizer](https://scikit-learn.org/stable/modules/generated/sklearn.feature_extraction.text.CountVectorizer.html)
and [HashingVectorizer](https://scikit-learn.org/stable/modules/generated/sklearn.feature_extraction.text.HashingVectorizer.html) 
though only a subset of features will be implemented.
