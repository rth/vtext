/*!
# vtext

[![CircleCI](https://circleci.com/gh/rth/vtext/tree/master.svg?style=svg)](https://circleci.com/gh/rth/vtext/tree/master)

NLP in Rust with Python bindings

This package aims to provide a high performance toolkit for ingesting textual data for
machine learning applications.

The API is currently unstable.

## Features

 - Tokenization: Regexp tokenizer, Unicode segmentation + language specific rules
 - Analyzers (*planned*): word and character n-grams, skip grams
 - Token counting: converting token counts to sparse matrices for use
   in machine learning libraries. Similar to `CountVectorizer` and
   `HashingVectorizer` in scikit-learn.
 - Feature weighting (*planned*): feature weighting based on document
   frequency (TF-IDF), feature normalization.

## Installation

Add the following to `Cargo.toml`,
```toml
[dependencies]
text-vectorize = {"git" = "https://github.com/rth/vtext"}
```

## Example

```rust

let documents = vec![
    String::from("the moon in the sky"),
    String::from("The sky sky sky is blue"),
];
```

*/

#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate seahash;
#[macro_use]
extern crate ndarray;
extern crate hashbrown;
extern crate sprs;

mod math;
pub mod tokenize;
pub mod vectorize;
