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
/// Unicode Segmentation tokenizer
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
    /// Tokenize a string
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
