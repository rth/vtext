// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

/*!
# Sentence tokenization module

For instance let's tokenize the following text
```rust
use vtext::tokenize::Tokenizer;
use vtext::tokenize_sentence::*;

let s = "Here is one. Here is another? Bang!! This trailing text is one more";
```

Using the Unicode sentence tokenizer we would get,
```rust
# use vtext::tokenize::Tokenizer;
# use vtext::tokenize_sentence::*;
# let s = "Here is one. Here is another? Bang!! This trailing text is one more";

let tokenizer = UnicodeSentenceTokenizer::default();
let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
assert_eq!(tokens, &["Here is one. ", "Here is another? ", "Bang!! ", "This trailing text is one more"]);
```
Here `UnicodeSentenceTokenizerParams` object is a thin wrapper around the
[unicode-segmentation](https://github.com/unicode-rs/unicode-segmentation) crate.

Using the Punctuation sentence tokenizer we would get,
```rust
# use vtext::tokenize::Tokenizer;
# use vtext::tokenize_sentence::*;
# let s = "Here is one. Here is another? Bang!! This trailing text is one more";

let tokenizer = PunctuationTokenizer::default();
let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
assert_eq!(tokens, &["Here is one. ", "Here is another? ", "Bang!", "! ", "This trailing text is one more"]);
```

Notice the "Bang!!" is treated differently.

*/

extern crate regex;
extern crate unicode_segmentation;

#[cfg(feature = "python")]
use dict_derive::{FromPyObject, IntoPyObject};
use unicode_segmentation::UnicodeSegmentation;

use crate::errors::VTextError;
use crate::tokenize::Tokenizer;

use std::fmt;

#[cfg(test)]
mod tests;

/// Unicode sentence tokenizer
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
    fn tokenize<'a>(&self, text: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
        Box::new(text.split_sentence_bounds())
    }
}

/// Punctuation sentence tokenizer
///
/// This simple tokenizer uses punctuation (default ".", "?", "!") to determine sentence boundaries.
/// Trailing whitespace is also captured in the preceding sentence.
///
/// # Arguments (PunctuationTokenizerParams)
///
/// * `punctuation` - a vector of punctuation tokens used to determine boundaries. Only the first "character" using the `chars` method is used.
///
/// * `whitespace` - a vector of whitespace tokens used to determine trailing sentence whitespace. Only the first "character" using the `chars` method is used.
///
#[derive(Clone)]
pub struct PunctuationTokenizer {
    pub params: PunctuationTokenizerParams,
}

/// Builder for the punctuation sentence tokenizer
#[derive(Debug, Clone)]
#[cfg_attr(feature = "python", derive(FromPyObject, IntoPyObject))]
pub struct PunctuationTokenizerParams {
    punctuation: Vec<String>,
    whitespace: Vec<String>,
}

impl PunctuationTokenizerParams {
    pub fn punctuation(&mut self, punctuation: Vec<String>) -> PunctuationTokenizerParams {
        self.punctuation = punctuation.clone();
        self.clone()
    }
    pub fn whitespace(&mut self, whitespace: Vec<String>) -> PunctuationTokenizerParams {
        self.whitespace = whitespace.clone();
        self.clone()
    }
    pub fn build(&mut self) -> Result<PunctuationTokenizer, VTextError> {
        Ok(PunctuationTokenizer {
            params: self.clone(),
        })
    }
}

macro_rules! vecString {
    ($( $char:expr ),*) => {{
        vec![
            $( $char.to_string(), )*
        ]
    }}
}

impl Default for PunctuationTokenizerParams {
    /// Create a new instance
    fn default() -> PunctuationTokenizerParams {
        PunctuationTokenizerParams {
            punctuation: vecString![".", "!", "?"],
            // Whitespace: Space, Tab, Line feed, Carriage return, Line tabulation and Form feed
            whitespace: vecString![" ", "\t", "\n", "\r", "\u{000B}", "\u{000C}"],
        }
    }
}

impl Default for PunctuationTokenizer {
    /// Create a new instance
    fn default() -> PunctuationTokenizer {
        PunctuationTokenizerParams::default().build().unwrap()
    }
}

impl fmt::Debug for PunctuationTokenizer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PunctuationTokenizer {{ punctuation: {:#?}, whitespace {:#?} }}",
            self.params.punctuation, self.params.whitespace
        )
    }
}

impl Tokenizer for PunctuationTokenizer {
    /// Tokenize a string
    fn tokenize<'a>(&'a self, text: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
        Box::new(punctuation_sentence_iterator(
            text,
            self.params.punctuation.clone(),
            self.params.whitespace.clone(),
        ))
    }
}

// Builder for PunctuationTokenizerIterator
fn punctuation_sentence_iterator<'a>(
    text: &'a str,
    punctuation: Vec<String>,
    whitespace: Vec<String>,
) -> PunctuationTokenizerIterator<'a> {

    let punctuation_chars: Vec<char> = punctuation.iter().map(|x| x.chars().next().unwrap()).collect();
    let whitespace_chars: Vec<char> = whitespace.iter().map(|x| x.chars().next().unwrap()).collect();

    PunctuationTokenizerIterator {
        text: text,
        punctuation: punctuation_chars,
        whitespace: whitespace_chars,
        seen_punct: false,
        i: 0,
        span_end: 0,
    }
}

// PunctuationTokenizerIterator internal state
struct PunctuationTokenizerIterator<'a> {
    text: &'a str,
    punctuation: Vec<char>,
    whitespace: Vec<char>,
    seen_punct: bool,
    i: usize,
    span_end: usize,
}

impl<'a> PunctuationTokenizerIterator<'a> {
    // Slice `self.text` using byte start and end indices. Returns a string slice.
    fn bytes_slice(&self, start: Option<usize>, end: Option<usize>) -> &'a str {
        let bytes_array: &[u8];
        if start.is_none() {
            bytes_array = &self.text.as_bytes()[..end.unwrap()];
        } else if end.is_none() {
            bytes_array = &self.text.as_bytes()[start.unwrap()..];
        } else if !end.is_none() & !start.is_none() {
            bytes_array = &self.text.as_bytes()[start.unwrap()..end.unwrap()];
        } else {
            return self.text;
        }
        std::str::from_utf8(bytes_array).unwrap()
    }
}

impl<'a> Iterator for PunctuationTokenizerIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let remaining_text = self.bytes_slice(Some(self.span_end), None);
        let idx_offset = self.span_end;

        // Process until a punctuation is encountered and trailing whitespace has finished
        for (i, char) in remaining_text.char_indices() {
            // idx_offset+i: bytes index of character
            self.i = i + idx_offset;
            let is_punct = self.punctuation.contains(&char);

            if self.seen_punct {
                let is_whitespace = self.whitespace.contains(&char);

                if !is_whitespace {
                    let span_start = self.span_end;
                    self.span_end = idx_offset + i;
                    self.seen_punct = false;
                    return Some(self.bytes_slice(Some(span_start), Some(self.span_end)));
                }
            } else if is_punct {
                self.seen_punct = true;
            }
        }

        // Trailing text
        if self.span_end < self.i {
            let span_start = self.span_end;
            self.span_end = self.i;
            return Some(self.bytes_slice(Some(span_start), None));
        }

        None
    }
}
