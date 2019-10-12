// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

#![feature(specialization)]

#[macro_use]
extern crate ndarray;
extern crate numpy;
#[macro_use]
extern crate pyo3;
extern crate rust_stemmers;
extern crate vtext;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod stem;
mod tokenize;
mod vectorize;

use vtext::metrics;

///  dice_similarity(x, y)
///
///  Sørensen–Dice similarity coefficient
///
///  This similarity tokenizes the input string x, y as 2-char n-grams,
///  into two sets of tokens X, Y then computes,
///
///  similarity(x, y) = 2 * |X ∩ Y| / (|X| + |Y|)
///
///  where |X| is the cardinality of set X.
///
///  Parameters
///  ----------
///  x : str
///     string to compare
///  y : str
///     string to compare
///
///  Result
///  ------
///  similarity : float
///     computed similarity
///
///  Example
///  -------
///  >>> dice_similarity('yesterday', 'today')
///  0.333..
#[pyfunction]
pub fn dice_similarity(x: &str, y: &str) -> PyResult<f64> {
    Ok(metrics::string::dice_similarity(x, y))
}

///  jaro_similarity(x, y)
///
///  Jaro similarity
///
///  The `Jaro
///  similarity <https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance#Jaro_Similarity>`_
///  accounts for the minimal number of character transpositions needed
///  to change one word in another.
///
///  References
///  ----------
///
///  Matthew A. Jaro (1989). Advances in record linkage methodology
///  as applied to the 1985 census of Tampa Florida. Journal of the
///  American Statistical Association. 84 (406): 414-20.
///
///  Parameters
///  ----------
///  x : str
///     string to compare
///  y : str
///     string to compare
///
///  Result
///  ------
///  similarity : float
///     computed similarity
///
///  Example
///  -------
///  >>> jaro_similarity('yesterday', 'today')
///  0.581..
#[pyfunction]
pub fn jaro_similarity(x: &str, y: &str) -> PyResult<f64> {
    Ok(metrics::string::jaro_similarity(x, y))
}

///  jaro_winkler_similarity(x, y, p, max_l)
///
///  Jaro Winkler similarity
///
///  The `Jaro-Winkler
///  similarity <https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance>`_
///  accounts for the minimal number of character transpositions needed
///  to change one word in another, and the length of the longest common prefix
///
///  The default values for parameters are ``p=0.1``, ``max_l=4``
///
///  References
///  ----------
///
///  William E. Winkler. 1990. String Comparator Metrics and Enhanced
///  Decision Rules in the Fellegi-Sunter Model of Record Linkage.
///  Proceedings of the Section on Survey Research Methods.
///  American Statistical Association: 354-359.
///
///  Parameters
///  ----------
///  x : str
///     string to compare
///  y : str
///     string to compare
///  p : float, default=0.1
///     the constant scaling factor to overweigh common prefixes
///  max_l : int, default=4
///     the length of common prefix at the start of the string
///
///  Result
///  ------
///  similarity : float
///     computed similarity
///
///  Example
///  -------
///  >>> jaro_winkler_similarity('yesterday', 'today')
///  0.581..
#[pyfunction]
pub fn jaro_winkler_similarity(x: &str, y: &str, p: f64, max_l: usize) -> PyResult<f64> {
    Ok(metrics::string::jaro_winkler_similarity(x, y, p, max_l))
}

///  edit_distance(x, y, substitution_cost, transpositions)
///
///  Levenshtein edit distance
///
///  It corresponds to the minimum number of single-character edits
///  (insertions, deletions, substitutions, and optionally transpositions)
///  required to change one word into the other.
///
///  Parameters
///  ----------
///  x : str
///     string to compare
///  y : str
///     string to compare
///  substitution_cost : int
///     the cost associated with one character substitutions
///  transpositions : bool
///     if True, transpositions are also taken into account
///
///  Result
///  ------
///  distance : float
///     computed distance
///
///  Example
///  -------
///  >>> edit_distance('yesterday', 'today')
///  4.0
#[pyfunction]
pub fn edit_distance(
    x: &str,
    y: &str,
    substitution_cost: usize,
    transpositions: bool,
) -> PyResult<f64> {
    Ok(metrics::string::edit_distance(
        x,
        y,
        substitution_cost,
        transpositions,
    ))
}

#[pymodule]
fn _lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<vectorize::_HashingVectorizerWrapper>()?;
    m.add_class::<vectorize::_CountVectorizerWrapper>()?;
    m.add_class::<tokenize::BaseTokenizer>()?;
    m.add_class::<tokenize::UnicodeSegmentTokenizer>()?;
    m.add_class::<tokenize::RegexpTokenizer>()?;
    m.add_class::<tokenize::VTextTokenizer>()?;
    m.add_class::<tokenize::CharacterTokenizer>()?;
    m.add_class::<stem::SnowballStemmer>()?;
    m.add_wrapped(wrap_pyfunction!(dice_similarity))?;
    m.add_wrapped(wrap_pyfunction!(jaro_similarity))?;
    m.add_wrapped(wrap_pyfunction!(jaro_winkler_similarity))?;
    m.add_wrapped(wrap_pyfunction!(edit_distance))?;
    Ok(())
}
