extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

/// Tokenize with Unicode Segmentation
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
    /// Tokenize a string
    pub fn tokenize<'a>(&self, text: &'a str) -> Vec<&'a str> {
        if self.word_bounds {
            let res = text.split_word_bounds().filter(|x| x != &" ");
            return res.collect::<Vec<&str>>();
        } else {
            return text.unicode_words().collect::<Vec<&str>>();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenize::UnicodeSegmentTokenizer;

    #[test]
    fn test_unicode_tokenizer() {
        let s = "The quick (\"brown\") fox can't jump 32.3 feet, right?";

        let tokenizer = UnicodeSegmentTokenizer { word_bounds: false };
        let tokens = tokenizer.tokenize(s);
        let b: &[_] = &[
            "The", "quick", "brown", "fox", "can't", "jump", "32.3", "feet", "right",
        ];
        assert_eq!(tokens, b);

        let tokenizer = UnicodeSegmentTokenizer { word_bounds: true };
        let tokens = tokenizer.tokenize(s);
        let b: &[_] = &[
            "The", "quick", "(", "\"", "brown", "\"", ")", "fox", "can't", "jump", "32.3", "feet",
            ",", "right", "?",
        ];
        assert_eq!(tokens, b);
    }
}
