extern crate regex;
extern crate unicode_segmentation;

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
        for tok in tokens {
            // Handle contractions
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
        let tokens_ref: &[_] = &["We", "ca", "n't"];
        assert_eq!(tokens, tokens_ref);

        let s = "it's";
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        let tokens_ref: &[_] = &["it", "'s"];
        assert_eq!(tokens, tokens_ref);

        let s = "it’s";
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        let tokens_ref: &[_] = &["it", "’s"];
        assert_eq!(tokens, tokens_ref);
    }

    #[test]
    fn test_vtext_tokenizer_all_lang() {
        let tokenizer = VTextTokenizer::new("en");

        let s = "name@domain.com";
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        let tokens_ref: &[_] = &["name@domain.com"];
        // TODO
        // assert_eq!(tokens, tokens_ref);

    }

}
