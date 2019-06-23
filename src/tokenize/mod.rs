// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

/*!
# Tokenization module

This module includes several tokenizers

For instance let's tokenize the following sentence,
```rust
use vtext::tokenize::*;

let s = "The “brown” fox can't jump 32.3 feet, right?";
```

Using a regular expression tokenizer we would get,
```rust
# let s = "The “brown” fox can't jump 32.3 feet, right?";
# use vtext::tokenize::*;
let tokenizer = RegexpTokenizerParams::default().build().unwrap();
let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
assert_eq!(tokens, &["The", "brown", "fox", "can", "jump", "32", "feet", "right"]);
```

which would remove all punctuation. A more general approach is to apply unicode segmentation,
```rust
# let s = "The “brown” fox can't jump 32.3 feet, right?";
# use vtext::tokenize::*;
let tokenizer = UnicodeSegmentTokenizerParams::default().build().unwrap();
let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
assert_eq!(tokens, &["The", "“", "brown", "”", "fox", "can't", "jump", "32.3", "feet", ",", "right", "?"]);
```
Here `UnicodeSegmentTokenizer` object is a thin wrapper around the
[unicode-segmentation](https://github.com/unicode-rs/unicode-segmentation) crate.

This approach produces better results, however for instance the word "can't" should be tokenized
as "ca", "n't" in English. To address such issues, we apply several additional rules on the previous results,

```rust
# let s = "The “brown” fox can't jump 32.3 feet, right?";
# use vtext::tokenize::*;
let tokenizer = VTextTokenizerParams::default().build().unwrap();
let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
assert_eq!(tokens, &["The", "“", "brown", "”", "fox", "ca", "n't", "jump", "32.3", "feet", ",", "right", "?"]);

*/
extern crate regex;
extern crate unicode_segmentation;

use crate::errors::VTextError;
use regex::Regex;
use std::fmt;
use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
mod tests;

pub trait Tokenizer: fmt::Debug {
    fn tokenize<'a>(&'a self, text: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a>;
}

/// Regular expression tokenizer
///
#[derive(Clone)]
pub struct RegexpTokenizer {
    pub pattern: String,
    regexp: Regex,
}

/// Builder for the regexp tokenizer
#[derive(Debug, Clone)]
pub struct RegexpTokenizerParams {
    pattern: String,
}

impl RegexpTokenizerParams {
    pub fn pattern(&mut self, value: &str) -> RegexpTokenizerParams {
        self.pattern = value.to_string();
        self.clone()
    }
    pub fn build(&mut self) -> Result<RegexpTokenizer, VTextError> {
        let pattern = &self.pattern;
        let regexp = Regex::new(pattern).unwrap();
        Ok(RegexpTokenizer { pattern: pattern.to_string(), regexp: regexp })
    }
}

impl Default for RegexpTokenizerParams {
    /// Create a new instance
    fn default () -> RegexpTokenizerParams {
        RegexpTokenizerParams { pattern : r"\b\w\w+\b".to_string() }
    }
}

impl Tokenizer for RegexpTokenizer {
    /// Tokenize a string
    fn tokenize<'a>(&'a self, text: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
        Box::new(self.regexp.find_iter(text).map(|m| m.as_str()))
    }
}

impl fmt::Debug for RegexpTokenizer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RegexpTokenizer {{ pattern:  {} }}", self.pattern)
    }
}

/// Unicode Segmentation tokenizer
///
/// This implementation is a thin wrapper around the
/// `unicode-segmentation` crate
///
/// ## References
///
/// * [Unicode® Standard Annex #29](http://www.unicode.org/reports/tr29/)
#[derive(Debug, Clone)]
pub struct UnicodeSegmentTokenizer {
    pub word_bounds: bool,
}

/// Builder for the unicode segmentation tokenizer
#[derive(Debug, Clone)]
pub struct UnicodeSegmentTokenizerParams {
    params: UnicodeSegmentTokenizer,
}

impl UnicodeSegmentTokenizerParams {
    pub fn word_bounds(&mut self, value: bool) -> UnicodeSegmentTokenizerParams {
        self.params.word_bounds = value;
        self.clone()
    }
    pub fn build(&mut self) -> Result<UnicodeSegmentTokenizer, VTextError> {
        Ok(self.params.clone())
    }
}

impl Default for UnicodeSegmentTokenizerParams {
    fn default() -> UnicodeSegmentTokenizerParams {
        UnicodeSegmentTokenizerParams {
            params: UnicodeSegmentTokenizer { word_bounds: true },
        }
    }
}

impl Tokenizer for UnicodeSegmentTokenizer {
    /// Tokenize a string
    fn tokenize<'a>(&self, text: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
        if self.word_bounds {
            let res = text.split_word_bounds().filter(|x| x != &" ");
            Box::new(res)
        } else {
            Box::new(text.unicode_words())
        }
    }
}

/// vtext tokenizer
///
/// This tokenizer a few additional rules on top of word boundaries computed
/// by unicode segmentation.
///
/// Additional language specific rules are implemented for English (en),
/// and French (en). Providing `lang` parameter with any other value, will siletly
/// fallback to `lang="any"`.
///
///
/// ## References
///
/// * [Unicode® Standard Annex #29](http://www.unicode.org/reports/tr29/)
#[derive(Debug, Clone)]
pub struct VTextTokenizer {
    pub lang: String,
}

/// Builder for the VTextTokenizer
#[derive(Debug, Clone)]
pub struct VTextTokenizerParams {
    lang: String
}

impl VTextTokenizerParams {
    pub fn lang(&mut self, value: &str) -> VTextTokenizerParams {
        self.lang = value.to_string();
        self.clone()
    }
    pub fn build(&mut self) -> Result<VTextTokenizer, VTextError> {
        let lang = match &self.lang[..] {
            "en" | "fr" => &self.lang[..],
            _ => {
                // TODO: add some warning message here
                //println!(
                //    "Warning: Lokenizer for {} \
                //     is not implemented! Falling back to the \
                //     language independent tokenizer!",
                //    lang
                //);
                "any"
            }
        };
        Ok(VTextTokenizer {
            lang: lang.to_string(),
        })
    }
}



impl Default for VTextTokenizerParams {
    /// Create a new instance
    fn default() -> VTextTokenizerParams {
        VTextTokenizerParams {lang: "en".to_string()}
    }
}

impl Tokenizer for VTextTokenizer {
    /// Tokenize a string
    fn tokenize<'a>(&self, text: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
        let tokens = text.split_word_bounds();

        let mut res: Vec<&'a str> = Vec::new();

        let mut punct_start_seq: i64 = -1;
        let mut punct_last = 'X';
        let mut str_idx: usize = 0;

        for tok in tokens {
            let tok_len = tok.len();
            str_idx += tok_len;
            if (tok_len == 1) & (tok != " ") {
                // Handle punctuation
                let ch = tok.chars().next().unwrap();
                if ch.is_ascii_punctuation() {
                    if ch != punct_last {
                        if punct_start_seq >= 0 {
                            res.push(&text[punct_start_seq as usize..str_idx - tok_len]);
                        }
                        punct_start_seq = (str_idx as i64) - (tok_len as i64);
                    }
                    punct_last = ch;
                    continue;
                }
            }
            if punct_start_seq >= 0 {
                res.push(&text[punct_start_seq as usize..str_idx - tok_len]);
                punct_start_seq = -1;
                punct_last = 'X';
            }

            match self.lang.as_ref() {
                "en" => {
                    // Handle contractions
                    if let Some(apostroph_idx) = tok.find(&"'") {
                        let mut apostroph_idx = apostroph_idx;
                        if tok.ends_with(&"n't") {
                            // also include the "n" from "n't"
                            apostroph_idx -= 1;
                        }
                        res.push(&tok[..apostroph_idx]);
                        res.push(&tok[apostroph_idx..]);
                        continue;
                    } else if let Some(apostroph_idx) = tok.find(&"’") {
                        // TODO: refactor to avoid repetitions
                        let mut apostroph_idx = apostroph_idx;
                        if tok.ends_with(&"n’t") {
                            // also include the "n" from "n't"
                            apostroph_idx -= 1;
                        }
                        res.push(&tok[..apostroph_idx]);
                        res.push(&tok[apostroph_idx..]);
                        continue;
                    }
                }
                "fr" => {
                    // Handle English contractions
                    if let Some(apostroph_idx) = tok.find(&"'") {
                        let apostroph_idx = apostroph_idx;
                        if apostroph_idx == 1 {
                            let apostroph_idx = apostroph_idx + "'".len();
                            res.push(&tok[..apostroph_idx]);
                            res.push(&tok[apostroph_idx..]);
                            continue;
                        }
                    }
                }
                _ => {}
            };
            res.push(tok);

            if res.len() >= 3 {
                // Merge some sequences
                let tok0 = res[res.len() - 3];
                let tok1 = res[res.len() - 2];
                let tok2 = res[res.len() - 1];
                if (tok0 != " ") & (tok2 != " ") & !tok0.is_empty() & !tok2.is_empty() {
                    let char0_last = tok0.chars().last().unwrap();
                    let char2_first = tok0.chars().next().unwrap();
                    let f1 = ((tok1 == "-") | (tok1 == "@") | (tok1 == "&"))
                        & char0_last.is_alphanumeric()
                        & char2_first.is_alphanumeric();
                    let f2 = ((tok1 == "/") | (tok1 == ":"))
                        & char0_last.is_numeric()
                        & char2_first.is_numeric();

                    if f1 | f2 {
                        res.truncate(res.len() - 3);
                        res.push(&text[str_idx - tok0.len() - tok1.len() - tok2.len()..str_idx]);
                    }
                }
            }
        }

        if punct_start_seq >= 0 {
            res.push(&text[punct_start_seq as usize..]);
        }

        // remove whitespace tokens
        let res = res.into_iter().filter(|x| x != &" ");
        Box::new(res)
    }
}

/// Character tokenizer
#[derive(Debug, Clone)]
pub struct CharacterTokenizer {
    pub window_size: usize,
}

impl CharacterTokenizer {
    /// Create a new instance
    pub fn new(window_size: usize) -> CharacterTokenizer {
        CharacterTokenizer { window_size }
    }
}

impl Tokenizer for CharacterTokenizer {
    /// Tokenize a string
    fn tokenize<'a>(&self, text: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
        let res = text
            .char_indices()
            .zip(
                text.char_indices()
                    .skip(self.window_size)
                    .chain(Some((text.len(), ' '))),
            )
            .map(move |((i, _), (j, _))| &text[i..j]);
        Box::new(res)
    }
}
