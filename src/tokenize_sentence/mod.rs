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

You can easily customise the `PunctuationTokenizer` to work with other languages. For example,
```rust
# use vtext::tokenize::Tokenizer;
# use vtext::tokenize_sentence::*;
use vtext::vecString;

let s = "বৃহত্তম ভাষা। বাংলা";
let punctuation = vecString!['।'];

let tokenizer = PunctuationTokenizerParams::default().punctuation(punctuation).build().unwrap();
let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
assert_eq!(tokens, &["বৃহত্তম ভাষা। ", "বাংলা"]);
```

Refer to the [test cases](https://github.com/rth/vtext/blob/master/src/tokenize_sentence/tests.rs)
for further langauge examples.

*/

extern crate regex;
extern crate unicode_segmentation;

#[cfg(feature = "python")]
use dict_derive::{FromPyObject, IntoPyObject};
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

use crate::errors::EstimatorErr;
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", derive(FromPyObject, IntoPyObject))]
pub struct UnicodeSentenceTokenizerParams {}

impl UnicodeSentenceTokenizerParams {
    pub fn build(&mut self) -> Result<UnicodeSentenceTokenizer, EstimatorErr> {
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
/// * `punctuation` - Punctuation tokens used to determine boundaries. Only the first "character"
///                   using the `chars` method is used.
///
///
#[derive(Clone)]
pub struct PunctuationTokenizer {
    pub params: PunctuationTokenizerParams,
}

/// Builder for the punctuation sentence tokenizer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", derive(FromPyObject, IntoPyObject))]
pub struct PunctuationTokenizerParams {
    punctuation: Vec<String>,
}

impl PunctuationTokenizerParams {
    pub fn punctuation(&mut self, punctuation: Vec<String>) -> PunctuationTokenizerParams {
        self.punctuation = punctuation;
        self.clone()
    }
    pub fn build(&mut self) -> Result<PunctuationTokenizer, EstimatorErr> {
        Ok(PunctuationTokenizer {
            params: self.clone(),
        })
    }
}

#[macro_export]
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
            "PunctuationTokenizer {{ punctuation: {:#?} }}",
            self.params.punctuation
        )
    }
}

impl Tokenizer for PunctuationTokenizer {
    /// Tokenize a string
    fn tokenize<'a>(&'a self, text: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
        Box::new(punctuation_sentence_iterator(
            text,
            self.params.punctuation.clone(),
        ))
    }
}

// Builder for PunctuationTokenizerIterator
fn punctuation_sentence_iterator<'a>(
    text: &'a str,
    punctuation: Vec<String>,
) -> PunctuationTokenizerIterator<'a> {
    let punctuation_chars: Vec<char> = punctuation
        .iter()
        .map(|x| x.chars().next().unwrap())
        .collect();

    PunctuationTokenizerIterator {
        text,
        punctuation: punctuation_chars,
        seen_punct: false,
        i: 0,
        span_end: 0,
        bytes_len: text.as_bytes().len(),
    }
}

// PunctuationTokenizerIterator internal state
struct PunctuationTokenizerIterator<'a> {
    text: &'a str,
    punctuation: Vec<char>,
    seen_punct: bool,
    i: usize,
    span_end: usize,
    bytes_len: usize,
}

impl<'a> PunctuationTokenizerIterator<'a> {
    // Slice `self.text` using byte start and end indices. Returns a string slice.
    fn bytes_slice(&self, start: Option<usize>, end: Option<usize>) -> &'a str {
        // View string as bytes array
        let bytes = self.text.as_bytes();
        let bytes_span: &[u8];

        // Slice array
        if let Some(start_idx) = start {
            if let Some(end_idx) = end {
                bytes_span = &bytes[start_idx..end_idx];
            } else {
                bytes_span = &bytes[start_idx..];
            }
        } else if let Some(end_idx) = end {
            bytes_span = &bytes[..end_idx];
        } else {
            return self.text;
        }

        // View slice as str
        std::str::from_utf8(bytes_span).unwrap()
    }
}

impl<'a> Iterator for PunctuationTokenizerIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.span_end >= self.bytes_len {
            return None;
        }

        let remaining_text = self.bytes_slice(Some(self.span_end), None);
        let idx_offset = self.span_end;

        // Process until a punctuation is encountered and trailing whitespace has finished
        for (i, character) in remaining_text.char_indices() {
            // idx_offset+i: bytes index of character
            self.i = i + idx_offset;
            let is_punct = self.punctuation.contains(&character);

            if self.seen_punct {
                let is_whitespace = character.is_whitespace();

                if !is_whitespace {
                    let span_start = self.span_end;
                    self.span_end = idx_offset + i;
                    self.seen_punct = false;
                    let span = self.bytes_slice(Some(span_start), Some(self.span_end));
                    if !span.is_empty() {
                        // Dont output if bytes represent 0 characters
                        return Some(span);
                    }
                }
            } else if is_punct {
                self.seen_punct = true;
            }
        }

        // Trailing text
        if self.span_end < self.bytes_len {
            let span_start = self.span_end;
            self.span_end = self.bytes_len;
            let span = self.bytes_slice(Some(span_start), None);
            if !span.is_empty() {
                // Dont output if bytes represent 0 characters
                return Some(span);
            }
        }

        None
    }
}
