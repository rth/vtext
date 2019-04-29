// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

/*!
String metrics

*/
use hashbrown::HashSet;
use itertools::Itertools;
use ndarray::{Array, ShapeBuilder};
use std::cmp::{max, min};
use std::iter::FromIterator;

///  Levenshtein edit distance
///
///  # Example
///  ```rust
///  use vtext::metrics::string::edit_distance;
///
///  let res = edit_distance("yesterday", "today", 1, false);
///  // returns 5.0
///  ```
pub fn edit_distance(x: &str, y: &str, substitution_cost: usize, transpositions: bool) -> f64 {
    // implementation adapted from NLTK

    let x_len = x.chars().count();
    let y_len = y.chars().count();

    // initialize the 2D array
    // TODO: there is likely a way to avoid allocating this array
    let mut lev = Array::<i32, _>::zeros((x_len + 1, y_len + 1).f());
    for idx in 1..x_len + 1 {
        lev[[idx, 0]] = idx as i32
    }
    for idx in 1..y_len + 1 {
        lev[[0, idx]] = idx as i32
    }

    for (x_idx, c1) in x.chars().enumerate() {
        for (y_idx, c2) in y.chars().enumerate() {
            // skipping a character in x
            let a = lev[[x_idx, y_idx + 1]] + 1;
            // skipping a character in y
            let b = lev[[x_idx + 1, y_idx]] + 1;

            // substitution
            let mut c = lev[[x_idx, y_idx]];
            if c1 != c2 {
                c += substitution_cost as i32;
            }

            // pick the cheapest
            c = min(min(a, b), c);

            if transpositions {
                if (x_idx > 1) & (y_idx > 1) {
                    if (x.chars().nth(x_idx - 1).unwrap() == c2)
                        & (y.chars().nth(y_idx - 1).unwrap() == c1)
                    {
                        c = min(c, lev[[x_idx - 1, y_idx - 1]] + 1);
                    }
                }
            }
            lev[[x_idx + 1, y_idx + 1]] = c;
        }
    }
    lev[[x_len, y_len]] as f64
}

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
    } else if x == y {
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
///
///  The [Jaro
///  similarity](https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance#Jaro_Similarity)
///  accounts for the minimal number of character transpositions needed
///  to change one word in another.
///
///  # References
///
///  Matthew A. Jaro (1989). Advances in record linkage methodology
///  as applied to the 1985 census of Tampa Florida. Journal of the
///  American Statistical Association. 84 (406): 414-20.
///
///
///  # Example
///  ```rust
///  use vtext::metrics::string::jaro_similarity;
///
///  let res = jaro_similarity("yesterday", "today");
///  // returns 0.581
///  ```
pub fn jaro_similarity(x: &str, y: &str) -> f64 {
    // implementation adapted from NLTK
    let x_chars: Vec<char> = x.chars().collect::<Vec<char>>();
    let y_chars: Vec<char> = y.chars().collect::<Vec<char>>();
    let x_len = x_chars.len();
    let y_len = y_chars.len();

    // The upper bound of the distance for being a matched character.
    let match_bound = max(x_len, y_len);
    // no.of matched characters in s1 and s2
    // no. of transpositions between s1 and s2
    let mut transpositions = 0;
    // positions in s1 which are matches to some character in s2
    let mut flagged_1: Vec<usize> = Vec::with_capacity(5);
    // positions in s2 which are matches to some character in s1
    let mut flagged_2: Vec<usize> = Vec::with_capacity(5);
    for (x_idx, x_char) in x_chars.iter().enumerate() {
        let upperbound = min(x_idx + match_bound, y_len - 1);
        let lowerbound = max(0, x_idx as i32 - match_bound as i32) as usize;
        for j in lowerbound..upperbound + 1 {
            if (x_char == &y_chars[j]) & !flagged_2.contains(&j) {
                flagged_1.push(x_idx);
                flagged_2.push(j);
                break;
            }
        }
    }
    flagged_2.sort_unstable();

    let matches = flagged_1.len();

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
        / 3.0
}

///  Jaro Winkler similarity
///
///  The [Jaro-Winkler
///  similarity](https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance)
///  accounts for the minimal number of character transpositions needed
///  to change one word in another, and the length of the longest common prefix
///
///  The default values for parameters are p=0.1, max_l=4
///
///  # References
///
///  William E. Winkler. 1990. String Comparator Metrics and Enhanced
///  Decision Rules in the Fellegi-Sunter Model of Record Linkage.
///  Proceedings of the Section on Survey Research Methods.
///  American Statistical Association: 354-359.
///
///
///  # Example
///  ```rust
///  use vtext::metrics::string::jaro_winkler_similarity;
///
///  let res = jaro_winkler_similarity("yesterday", "today", 0.1, 4);
///  // returns 0.581
///  ```
pub fn jaro_winkler_similarity(x: &str, y: &str, p: f64, max_l: usize) -> f64 {
    // implementation adapted from NLTK
    //
    if (p * max_l as f64 <= 0.0) | (p * max_l as f64 >= 1.0) {
        panic!("{} not in (0, 1)!", p * max_l as f64)
    }

    let jaro_sim = jaro_similarity(x, y);

    // Compute the length of the common prefix
    let mut l = 0;
    for (s1_i, s2_i) in x.chars().zip(y.chars()) {
        if s1_i == s2_i {
            l += 1;
            if l == max_l {
                break;
            }
        } else {
            break;
        }
    }
    jaro_sim + (l as f64 * p * (1.0 - jaro_sim))
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

        let res = jaro_similarity("SHACKLEFORD", "SHACKELFORD");
        assert_eq!((res * 1000.).round() / 1000., 0.970);

        assert_eq!(jaro_similarity("", ""), 0.0);
        assert_eq!(jaro_similarity("1", "2"), 0.0);
        assert_eq!(jaro_similarity("test", "test"), 1.0);
    }

    #[test]
    fn test_jaro_winkler_similarity() {
        let res = jaro_winkler_similarity("SHACKLEFORD", "SHACKELFORD", 0.1, 4);
        assert_eq!((res * 1000.).round() / 1000., 0.982);

        assert_eq!(jaro_winkler_similarity("", "", 0.1, 4), 0.0);
        assert_eq!(jaro_winkler_similarity("1", "2", 0.1, 4), 0.0);
        assert_eq!(jaro_winkler_similarity("test", "test", 0.1, 4), 1.0);
    }

    #[test]
    #[should_panic]
    fn test_jaro_winkler_similarity_invalid() {
        // Should panic: 0.5*4 > 1
        jaro_winkler_similarity("AABABCAAAC", "ABAACBAAAC", 0.5, 4);
    }

    #[test]
    fn test_edit_distance() {
        let res = edit_distance("yesterday", "today", 1, false);
        assert_eq!(res, 5.0);
    }

}
