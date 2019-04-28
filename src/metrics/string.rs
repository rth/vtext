/*!
String metrics

*/
use std::collections::HashSet;
use std::iter::FromIterator;

///  Sørensen–Dice similarity coefficient
///
///  Uses 2-char n-grams by default.
///
///  # Example
///  ```rust
///  use vtext::metrics::string::dice_similarity;
///
///  let res = dice_similarity("yesterday", "today");
///  assert_eq!(res, 1.0);
///  ```
pub fn dice_similarity(x: &str, y: &str) -> f64 {
    let mut x_tokens = Vec::new();

    for ngram in x.chars().collect::<Vec<char>>().windows(2) {
        x_tokens.push(ngram.to_owned());
    }

    let mut y_tokens = Vec::new();

    for ngram in y.chars().collect::<Vec<char>>().windows(2) {
        y_tokens.push(ngram.to_owned());
    }

    let x_set: HashSet<&Vec<char>> = HashSet::from_iter(x_tokens.iter());
    let y_set: HashSet<&Vec<char>> = HashSet::from_iter(y_tokens.iter());

    let intersection_len = x_set.intersection(&y_set).count();

    (2 * intersection_len) as f64 / (x_set.len() + y_set.len()) as f64
}
