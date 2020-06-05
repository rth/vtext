// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

/*!
# Sentence tokenization module

For instance let's tokenize the following text using the Unicode segmentation
```rust
let s = "Here is one. Here is another! This trailing text is one more";
use vtext::tokenize::*;
use vtext::tokenize_sentence::*;
let tokenizer = UnicodeSentenceTokenizer::default();
let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
assert_eq!(tokens, &["Here is one. ", "Here is another! ", "This trailing text is one more"];);
```
Here `UnicodeSentenceTokenizerParams` object is a thin wrapper around the
[unicode-segmentation](https://github.com/unicode-rs/unicode-segmentation) crate.

*/


extern crate regex;
extern crate unicode_segmentation;

use crate::errors::VTextError;
#[cfg(feature = "python")]
use dict_derive::{FromPyObject, IntoPyObject};
use crate::tokenize::Tokenizer;
use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
mod tests;

/// Unicode Sentence tokenizer
///
/// This implementation is a thin wrapper around the
/// `unicode-segmentation` crate
///
/// ## References
///
/// * [Unicode® Standard Annex #29](http://www.unicode.org/reports/tr29/)
#[derive(Debug, Clone)]
pub struct UnicodeSentenceTokenizer {
    pub params: UnicodeSentenceTokenizerParams,
}

/// Builder for the unicode segmentation tokenizer
#[derive(Debug, Clone)]
#[cfg_attr(feature = "python", derive(FromPyObject, IntoPyObject))]
pub struct UnicodeSentenceTokenizerParams {}

impl UnicodeSentenceTokenizerParams {
    pub fn build(&mut self) -> Result<UnicodeSentenceTokenizer, VTextError> {
        Ok(UnicodeSentenceTokenizer {
            params: self.clone(),
        })
    }
}

impl Default for UnicodeSentenceTokenizerParams {
    fn default() -> UnicodeSentenceTokenizerParams {
        UnicodeSentenceTokenizerParams {}
    }
}

impl Default for UnicodeSentenceTokenizer {
    /// Create a new instance
    fn default() -> UnicodeSentenceTokenizer {
        UnicodeSentenceTokenizerParams::default().build().unwrap()
    }
}

impl Tokenizer for UnicodeSentenceTokenizer {
    /// Tokenize a string
    fn tokenize<'a>(&self, text: &'a str) -> Box<dyn Iterator<Item=&'a str> + 'a> {
        Box::new(text.split_sentence_bounds())
    }
}



