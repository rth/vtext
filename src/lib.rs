/*!
# vtext

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
 - Levenshtein edit distance; Sørensen-Dice, Jaro, Jaro Winkler string similarities

# Example

A simple tokenization example can be found below,
```rust
extern crate vtext;

use vtext::tokenize::VTextTokenizer;

let tok = VTextTokenizer::new("en");
let tokens = tok.tokenize("Flights can't depart after 2:00 pm.");

// returns &["Flights", "ca", "n't", "depart", "after", "2:00", "pm", "."]
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
#[macro_use]
extern crate itertools;

mod math;
pub mod metrics;
pub mod tokenize;
pub mod vectorize;
