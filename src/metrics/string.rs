/*!
String metrics

*/
use hashbrown::HashSet;
use itertools::Itertools;
use std::cmp::{max, min};
use std::iter::FromIterator;

///  Sørensen–Dice similarity coefficient
///
///  This similarity tokenizes the input string x, y as 2-char n-grams,
///  into two sets of tokens X, Y then computes,
///
///  similarity(x, y) = 2 * |X ∩ Y| / (|X| + |Y|)
///
///  where |X| is the cardinality of set X.
///
///  # Example
///  ```rust
///  use vtext::metrics::string::dice_similarity;
///
///  let res = dice_similarity("yesterday", "today");
///  // returns 0.333
///  ```
pub fn dice_similarity(x: &str, y: &str) -> f64 {
    if (x.len() < 2) | (y.len() < 2) {
        0.0
    } else if (x == y) {
        1.0
    } else {
        let mut x_set: HashSet<(char, char)> = HashSet::with_capacity(5);

        for (char_1, char_2) in x.chars().tuple_windows() {
            x_set.insert((char_1, char_2));
        }

        let mut y_set: HashSet<(char, char)> = HashSet::with_capacity(x_set.len());

        for (char_1, char_2) in y.chars().tuple_windows() {
            y_set.insert((char_1, char_2));
        }

        let intersection_len = x_set.intersection(&y_set).count();

        (2 * intersection_len) as f64 / (x_set.len() + y_set.len()) as f64
    }
}

///  Jaro similarity
pub fn jaro_similarity(x: &str, y: &str) -> f64 {
    // implementation adapted from NLTK
    let x_chars: Vec<char> = x.chars().collect::<Vec<char>>();
    let y_chars: Vec<char> = y.chars().collect::<Vec<char>>();
    let x_len = x_chars.len();
    let y_len = y_chars.len();

    // The upper bound of the distance for being a matched character.
    let match_bound = max(x_len, y_len);
    // no.of matched characters in s1 and s2
    let mut matches = 0;
    // no. of transpositions between s1 and s2
    let mut transpositions = 0;
    // positions in s1 which are matches to some character in s2
    let mut flagged_1: Vec<usize> = Vec::new();
    // positions in s2 which are matches to some character in s1
    let mut flagged_2: Vec<usize> = Vec::new();
    for (x_idx, x_char) in x_chars.iter().enumerate() {
        let upperbound = min(x_idx + match_bound, y_len - 1);
        let lowerbound = max(0, x_idx as i32 - match_bound as i32) as usize;
        for j in (lowerbound..upperbound + 1) {
            if (x_char == &y_chars[j]) & !flagged_2.contains(&j) {
                matches += 1;
                flagged_1.push(x_idx);
                flagged_2.push(j);
                break;
            }
        }
        flagged_2.sort_unstable();
    }
    if matches == 0 {
        return 0.0;
    }
    for (i, j) in flagged_1.iter().zip(flagged_2.iter()) {
        if x_chars[*i] != y_chars[*j] {
            transpositions += 1
        }
    }
    (matches as f64 / x_len as f64
        + matches as f64 / y_len as f64
        + (matches as f64 - (transpositions / 2) as f64) / matches as f64)
        / 3 as f64
}

#[cfg(test)]
mod tests {
    use crate::metrics::string::*;

    #[test]
    fn test_dice_similarity() {
        let res = dice_similarity("yesterday", "today");
        assert_eq!((res * 100.).round() / 100., 0.33);

        assert_eq!(dice_similarity("healed", "sealed"), 0.8);

        assert_eq!(dice_similarity("", ""), 0.0);
        // 1 char, doesn't allow to make a single 2-char ngram
        assert_eq!(dice_similarity("1", "test"), 0.0);

        assert_eq!(dice_similarity("test", "test"), 1.0);
    }

    #[test]
    fn test_jaro_similarity() {
        let res = jaro_similarity("AABABCAAAC", "ABAACBAAAC");
        assert_eq!((res * 1000.).round() / 1000., 0.933);

        assert_eq!(jaro_similarity("", ""), 0.0);
        assert_eq!(jaro_similarity("1", "2"), 0.0);
    }

}
