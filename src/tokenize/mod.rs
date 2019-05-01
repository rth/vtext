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
let tokenizer = RegexpTokenizer::new(r"\b\w\w+\b".to_string());
let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
assert_eq!(tokens, &["The", "brown", "fox", "can", "jump", "32", "feet", "right"]);
```

which would remove all punctuation. A more general approach is to apply unicode segmentation,
```rust
# let s = "The “brown” fox can't jump 32.3 feet, right?";
# use vtext::tokenize::UnicodeSegmentTokenizer;
let tokenizer = UnicodeSegmentTokenizer::new(true);
let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
assert_eq!(tokens, &["The", "“", "brown", "”", "fox", "can't", "jump", "32.3", "feet", ",", "right", "?"]);
```
Here `UnicodeSegmentTokenizer` object is a thin wrapper around the
[unicode-segmentation](https://github.com/unicode-rs/unicode-segmentation) crate.

This approach produces better results, however for instance the word "can't" should be tokenized
as "ca", "n't" in English. To address such issues, we apply several additional rules on the previous results,

```rust
# let s = "The “brown” fox can't jump 32.3 feet, right?";
# use vtext::tokenize::VTextTokenizer;
let tokenizer = VTextTokenizer::new("en");
let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
assert_eq!(tokens, &["The", "“", "brown", "”", "fox", "ca", "n't", "jump", "32.3", "feet", ",", "right", "?"]);

*/
extern crate regex;
extern crate unicode_segmentation;

use itertools::Itertools;
use std::cmp;

use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
mod tests;

pub trait Tokenizer {
    fn tokenize<'a>(&'a self, text: &'a str) -> Box<Iterator<Item = &'a str> + 'a>;
}


/// Regular expression tokenizer
///
#[derive(Debug)]
pub struct RegexpTokenizer {
    pub pattern: String,
    regexp: Regex,
}

impl RegexpTokenizer {
    /// Create a new instance
    pub fn new(pattern: String) -> RegexpTokenizer {
        let regexp = Regex::new(&pattern).unwrap();

        RegexpTokenizer {
            pattern: pattern,
            regexp: regexp,
        }
    }
}

impl Tokenizer for RegexpTokenizer {
    /// Tokenize a string
    fn tokenize<'a>(&'a self, text: &'a str) -> Box<Iterator<Item = &'a str> + 'a> {
        Box::new(self.regexp.find_iter(text).map(|m| m.as_str()))
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
#[derive(Debug)]
pub struct UnicodeSegmentTokenizer {
    pub word_bounds: bool,
}

impl UnicodeSegmentTokenizer {
    /// Create a new instance
    pub fn new(word_bounds: bool) -> UnicodeSegmentTokenizer {
        UnicodeSegmentTokenizer {
            word_bounds: word_bounds,
        }
    }
    /// Tokenize a string
    pub fn tokenize<'a>(&self, text: &'a str) -> Box<Iterator<Item = &'a str> + 'a> {
        if self.word_bounds {
            let res = text.split_word_bounds().filter(|x| x != &" ");
            return Box::new(res);
        } else {
            return Box::new(text.unicode_words());
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
#[derive(Debug)]
pub struct VTextTokenizer {
    pub lang: String,
}

impl VTextTokenizer {
    /// Create a new instance
    pub fn new(lang: &str) -> VTextTokenizer {
        let lang_valid = match lang {
            "en" | "fr" => lang,
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
        VTextTokenizer {
            lang: lang_valid.to_string(),
        }
    }
    /// Tokenize a string
    pub fn tokenize<'a>(&self, text: &'a str) -> Box<Iterator<Item = &'a str> + 'a> {
        let tokens = text.split_word_bounds();

        let mut res: Vec<&'a str> = Vec::new();

        let mut punct_start_seq: i64 = -1;
        let mut punct_last = 'X';
        let mut str_idx: usize = 0;

        for (idx, tok) in tokens.enumerate() {
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
                            apostroph_idx = apostroph_idx - 1;
                        }
                        res.push(&tok[..apostroph_idx]);
                        res.push(&tok[apostroph_idx..]);
                        continue;
                    } else if let Some(apostroph_idx) = tok.find(&"’") {
                        // TODO: refactor to avoid repetitions
                        let mut apostroph_idx = apostroph_idx;
                        if tok.ends_with(&"n’t") {
                            // also include the "n" from "n't"
                            apostroph_idx = apostroph_idx - 1;
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
                if (tok0 != " ") & (tok2 != " ") & (tok0.len() > 0) & (tok2.len() > 0) {
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
        return Box::new(res);
    }
}

/// Character tokenizer
#[derive(Debug)]
pub struct CharacterTokenizer {
    pub window_size: usize,
}

impl CharacterTokenizer {
    /// Create a new instance
    pub fn new(window_size: usize) -> CharacterTokenizer {
        CharacterTokenizer {
            window_size: window_size,
        }
    }

    /// Tokenize a string
    pub fn tokenize<'a>(&self, text: &'a str) -> Box<Iterator<Item = &'a str> + 'a> {
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
