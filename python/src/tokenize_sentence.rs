// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

use pyo3::prelude::*;
use pyo3::types::PyList;

use vtext::tokenize::Tokenizer;

use vtext::tokenize_sentence::*;

use crate::tokenize::BaseTokenizer;

/// __init__(self, word_bounds=True)
///
/// Unicode sentence tokenizer
///
/// This implementation is a thin wrapper around the
/// `unicode-segmentation <https://github.com/unicode-rs/unicode-segmentation>`_
/// crate.
///
/// References
/// ----------
/// - `Unicode® Standard Annex #29 <http://www.unicode.org/reports/tr29/>`_
#[pyclass(extends=BaseTokenizer)]
pub struct UnicodeSentenceTokenizer {
    inner: vtext::tokenize_sentence::UnicodeSentenceTokenizer,
}

#[pymethods]
impl UnicodeSentenceTokenizer {
    #[new]
    fn new() -> (Self, BaseTokenizer) {
        let tokenizer = vtext::tokenize_sentence::UnicodeSentenceTokenizerParams::default()
            .build()
            .unwrap();

        (
            UnicodeSentenceTokenizer { inner: tokenizer },
            BaseTokenizer::new(),
        )
    }

    /// tokenize(self, x)
    ///
    /// Tokenize a string of sentences
    ///
    /// Parameters
    /// ----------
    /// x : bool
    ///   the string to tokenize
    ///
    /// Returns
    /// -------
    /// tokens : List[str]
    ///    computed tokens
    fn tokenize<'py>(&self, py: Python<'py>, x: &str) -> PyResult<&'py PyList> {
        let res: Vec<&str> = self.inner.tokenize(x).collect();
        let list = PyList::new(py, res);
        Ok(list)
    }

    /// get_params(self, x)
    ///
    /// Get parameters for this estimator.
    ///
    /// Returns
    /// -------
    /// params : mapping of string to any
    ///          Parameter names mapped to their values.
    fn get_params(&self) -> PyResult<UnicodeSentenceTokenizerParams> {
        Ok(self.inner.params.clone())
    }
}

// TODO: unsure how to import from `vtext::tokenize_sentence::vecString`
#[macro_export]
macro_rules! vecString {
    ($( $char:expr ),*) => {{
        vec![
            $( $char.to_string(), )*
        ]
    }}
}

/// __init__(self, punctuation=[".", "?", "!"], whitespace=[" ", "\t", "\n", "\r", "\u000B", "\u000C"])
///
/// Punctuation sentence tokenizer
///
/// This simple tokenizer uses punctuation (default ".", "?", "!") to determine sentence boundaries.
/// Trailing whitespace is also captured in the preceding sentence.
///
/// Parameters
/// ----------
/// punctuation : List[str]
///   Punctuation tokens used to determine boundaries. Only the first unicode "character" is used.
///
/// whitespace : List[str]
///   Whitespace tokens used to determine trailing sentence whitespace. Only the first unicode "character" is used.
///
#[pyclass(extends=BaseTokenizer)]
pub struct PunctuationTokenizer {
    inner: vtext::tokenize_sentence::PunctuationTokenizer,
}

#[pymethods]
impl PunctuationTokenizer {
    #[new]
    #[args(punctuation = "vecString![\".\", \"!\", \"?\"]", whitespace = "vecString![\" \", \"\\t\", \"\\n\", \"\\r\", \"\\u{000B}\", \"\\u{000C}\"]")]
    fn new(punctuation: Vec<String>, whitespace: Vec<String>) -> (Self, BaseTokenizer) {
        let tokenizer = vtext::tokenize_sentence::PunctuationTokenizerParams::default()
            .punctuation(punctuation)
            .whitespace(whitespace)
            .build()
            .unwrap();

        (
            PunctuationTokenizer { inner: tokenizer },
            BaseTokenizer::new(),
        )
    }

    /// tokenize(self, x)
    ///
    /// Tokenize a string of sentences
    ///
    /// Parameters
    /// ----------
    /// x : bool
    ///   the string to tokenize
    ///
    /// Returns
    /// -------
    /// tokens : List[str]
    ///    computed tokens
    fn tokenize<'py>(&self, py: Python<'py>, x: &str) -> PyResult<&'py PyList> {
        let res: Vec<&str> = self.inner.tokenize(x).collect();
        let list = PyList::new(py, res);
        Ok(list)
    }

    /// get_params(self, x)
    ///
    /// Get parameters for this estimator.
    ///
    /// Returns
    /// -------
    /// params : mapping of string to any
    ///          Parameter names mapped to their values.
    fn get_params(&self) -> PyResult<PunctuationTokenizerParams> {
        Ok(self.inner.params.clone())
    }
}
