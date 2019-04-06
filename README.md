# vtext

[![CircleCI](https://circleci.com/gh/rth/vtext/tree/master.svg?style=svg)](https://circleci.com/gh/rth/vtext/tree/master)

Fast NLP in Rust with Python bindings

This package aims to provide a convenient and high performance toolkit for ingesting textual data for
machine learning applications. Making existing NLP Rust crates accesible in Python with a common API is another goal of this project.

The API is currently unstable.

### Features

 - Tokenization: Regexp tokenizer, Unicode segmentation
 - Stemming: Snowball (in Python 15-20x faster than NLTK)
 - Analyzers (*planned*): word and character n-grams, skip grams
 - Token counting: converting token counts to sparse matrices for use
   in machine learning libraries. Similar to `CountVectorizer` and
   `HashingVectorizer` in scikit-learn.
 - Feature weighting (*planned*): feature weighting based on document
   frequency (TF-IDF), supervised term weighting (e.g. TF-IGM),
   feature normalization

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
