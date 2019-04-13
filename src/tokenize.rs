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
        match lang {
            "en" | "fr" => {}
            _ => {
                // TODO: add some warning message here
                //println!(
                //    "Warning: Lokenizer for {} \
                //     is not implemented! Falling back to the \
                //     language independent tokenizer!",
                //    lang
                //);
            }
        };
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
                // merge on dashes, /, or @
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
    fn test_vtext_tokenizer_all_lang() {
        let tokenizer = VTextTokenizer::new("en");

        for (s, tokens_ref) in [
            // float numbers
            ("23.2 meters", vec!["23.2", "meters"]),
            ("11,2 m", vec!["11,2", "m"]),
            // repeated punctuation
            ("1 ..", vec!["1", ".."]),
            ("I ...", vec!["I", "..."]),
            (", o ! o", vec![",", "o", "!", "o"]),
            ("... ok.", vec!["...", "ok", "."]),
            // dash separated words
            ("porte-manteau", vec!["porte-manteau"]),
            // emails
            ("name@domain.com", vec!["name@domain.com"]),
            // fractions
            ("1/2", vec!["1/2"]),
            ("and/or", vec!["and", "/", "or"]),
            // time
            ("8:30", vec!["8:30"]),
            ("B&B", vec!["B&B"]),
            // TODO ("Hello :)", vec!["Hello", ":)"])
            // TODO ("http://www.youtube.com/watch?v=q2lDF0XU3NI",
            // vec!["http://www.youtube.com/watch?v=q2lDF0XU3NI"])
        ]
        .iter()
        {
            let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
            assert_eq!(&tokens, tokens_ref);
        }
    }

    #[test]
    fn test_vtext_tokenizer_en() {
        let tokenizer = VTextTokenizer::new("en");

        for (s, tokens_ref) in [
            ("We can't", vec!["We", "ca", "n't"]),
            ("it's", vec!["it", "'s"]),
            ("it’s", vec!["it", "’s"]),
            // TODO ("N.Y.", vec!["N.Y."])
        ]
        .iter()
        {
            let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
            assert_eq!(&tokens, tokens_ref);
        }
    }

    #[test]
    fn test_vtext_tokenizer_fr() {
        let tokenizer = VTextTokenizer::new("fr");

        for (s, tokens_ref) in [("l'image", vec!["l'", "image"])].iter() {
            let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
            assert_eq!(&tokens, tokens_ref);
        }
    }

}
