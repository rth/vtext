// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

/*!
# vtext

NLP in Rust with Python bindings

This package aims to provide a high performance toolkit for ingesting textual data for
machine learning applications.

The API is currently unstable.

## Features

 - Tokenization: Regexp tokenizer, Unicode segmentation + language specific rules
 - Token counting: converting token counts to sparse matrices for use
   in machine learning libraries. Similar to `CountVectorizer` and
   `HashingVectorizer` in scikit-learn but will less broad functionality.
 - Levenshtein edit distance; Sørensen-Dice, Jaro, Jaro Winkler string similarities

# Example

A simple tokenization example can be found below,
```rust
extern crate vtext;

use vtext::tokenize::{VTextTokenizerParams,Tokenizer};

let tok = VTextTokenizerParams::default().lang("en").build().unwrap();
let tokens = tok.tokenize("Flights can't depart after 2:00 pm.");

// returns &["Flights", "ca", "n't", "depart", "after", "2:00", "pm", "."]
```

*/

#![allow(non_snake_case)]

pub mod errors;
mod math;
pub mod metrics;
pub mod tokenize;
pub mod tokenize_sentence;
pub mod token_processor;
pub mod vectorize;
