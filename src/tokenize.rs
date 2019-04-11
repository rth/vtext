extern crate regex;
extern crate unicode_segmentation;

use std::cmp;

use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

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
    /// Tokenize a string
    pub fn tokenize<'a>(&'a self, text: &'a str) -> impl Iterator<Item = &'a str> {
        self.regexp.find_iter(text).map(|m| m.as_str())
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
/// This tokenizer builds upon the `unicode-segmentation` crate
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
        VTextTokenizer {
            lang: lang.to_string(),
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

            if &self.lang == "en" {
                // Handle English contractions
                if let Some(apostroph_idx) = tok.find(&"'") {
                    let mut apostroph_idx = apostroph_idx;
                    if tok.ends_with(&"n't") {
                        // also include the "n" from "n't"
                        apostroph_idx = apostroph_idx - 1;
                    }
                    res.push(&tok[..apostroph_idx]);
                    res.push(&tok[apostroph_idx..]);
                } else if let Some(apostroph_idx) = tok.find(&"’") {
                    let mut apostroph_idx = apostroph_idx;
                    if tok.ends_with(&"n’t") {
                        // also include the "n" from "n't"
                        apostroph_idx = apostroph_idx - 1;
                    }
                    res.push(&tok[..apostroph_idx]);
                    res.push(&tok[apostroph_idx..]);
                } else {
                    res.push(tok);
                }
            } else {
                res.push(tok);
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

#[cfg(test)]
mod tests {
    use crate::tokenize::{RegexpTokenizer, UnicodeSegmentTokenizer, VTextTokenizer};

    #[test]
    fn test_regexp_tokenizer() {
        let s = "fox can't jump 32.3 feet, right?";

        let tokenizer = RegexpTokenizer::new(r"\b\w\w+\b".to_string());
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        let b: &[_] = &["fox", "can", "jump", "32", "feet", "right"];
        assert_eq!(tokens, b);
    }

    #[test]
    fn test_unicode_tokenizer() {
        let s = "The quick (\"brown\") fox can't jump 32.3 feet, right?";

        let tokenizer = UnicodeSegmentTokenizer { word_bounds: false };
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        let b: &[_] = &[
            "The", "quick", "brown", "fox", "can't", "jump", "32.3", "feet", "right",
        ];
        assert_eq!(tokens, b);

        let tokenizer = UnicodeSegmentTokenizer { word_bounds: true };
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        let b: &[_] = &[
            "The", "quick", "(", "\"", "brown", "\"", ")", "fox", "can't", "jump", "32.3", "feet",
            ",", "right", "?",
        ];
        assert_eq!(tokens, b);
    }

    #[test]
    fn test_vtext_tokenizer_en() {
        let tokenizer = VTextTokenizer::new("en");

        let s = "We can't";
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        assert_eq!(tokens, &["We", "ca", "n't"]);

        let s = "it's";
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        assert_eq!(tokens, &["it", "'s"]);

        let s = "it’s";
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        assert_eq!(tokens, &["it", "’s"]);

        let s = "N.Y.";
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        // TODO
        // assert_eq!(tokens, &["N.Y."]);
    }

    #[test]
    fn test_vtext_tokenizer_all_lang() {
        let tokenizer = VTextTokenizer::new("en");

        // float numbers
        let s = "23.2 meters";
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        assert_eq!(tokens, &["23.2", "meters"]);

        let s = "11,2 meters";
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        assert_eq!(tokens, &["11,2", "meters"]);

        // repeated punctuation
        let s = "1 ..";
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        assert_eq!(tokens, &["1", ".."]);

        let s = "I ...";
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        assert_eq!(tokens, &["I", "..."]);

        let s = ", o ! o";
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        assert_eq!(tokens, &[",", "o", "!", "o"]);

        let s = "... ok.";
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        assert_eq!(tokens, &["...", "ok", "."]);

        // dash separated words
        let s = "porte-manteau";
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        // TODO
        //assert_eq!(tokens, &["porte-manteau"]);

        let s = "name@domain.com";
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        // TODO
        // assert_eq!(tokens, &["name@domain.com"]);
        //
        let s = "Hello :)";
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        // TODO
        //assert_eq!(tokens, &["Hello", ":)"]);
    }

}
