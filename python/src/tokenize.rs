// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

use pyo3::prelude::*;
use pyo3::types::PyList;

use crate::utils::{deserialize_params, serialize_params};
use vtext::tokenize::*;

#[pyclass(module = "vtext.tokenize")]
pub struct BaseTokenizer {}

#[pymethods]
impl BaseTokenizer {
    #[new]
    pub fn new() -> Self {
        BaseTokenizer {}
    }
}

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
#[pyclass(extends=BaseTokenizer, module="vtext.tokenize")]
pub struct UnicodeWordTokenizer {
    inner: vtext::tokenize::UnicodeWordTokenizer,
}

#[pymethods]
impl UnicodeWordTokenizer {
    #[new]
    #[args(word_bounds = true)]
    fn new(word_bounds: bool) -> (Self, BaseTokenizer) {
        let tokenizer = vtext::tokenize::UnicodeWordTokenizerParams::default()
            .word_bounds(word_bounds)
            .build()
            .unwrap();

        (
            UnicodeWordTokenizer { inner: tokenizer },
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
    fn get_params(&self) -> PyResult<UnicodeWordTokenizerParams> {
        Ok(self.inner.params.clone())
    }

    pub fn __getstate__(&self, py: Python) -> PyResult<PyObject> {
        serialize_params(&self.inner.params, py)
    }

    pub fn __setstate__(&mut self, py: Python, state: PyObject) -> PyResult<()> {
        let mut params: UnicodeWordTokenizerParams = deserialize_params(py, state)?;
        self.inner = params.build().unwrap();
        Ok(())
    }
}

/// __init__(self, lang="en")
///
/// VText tokenizer
///
/// This tokenizer a few additional rules on top of word boundaries computed
/// by unicode segmentation.
///
/// Additional language specific rules are implemented for English (en),
/// and French (en). Providing `lang` parameter with any other value, will silently
/// fall back to ``lang='any'``.
///
///
/// References
/// ----------
///
/// - `Unicode® Standard Annex #29 <http://www.unicode.org/reports/tr29/>`_
#[pyclass(extends=BaseTokenizer, module="vtext.tokenize")]
pub struct VTextTokenizer {
    inner: vtext::tokenize::VTextTokenizer,
}

#[pymethods]
impl VTextTokenizer {
    #[new]
    #[args(lang = "\"en\"")]
    fn new(lang: &str) -> (Self, BaseTokenizer) {
        let tokenizer = vtext::tokenize::VTextTokenizerParams::default()
            .lang(lang)
            .build()
            .unwrap();

        (VTextTokenizer { inner: tokenizer }, BaseTokenizer::new())
    }

    /// tokenize(self, x)
    ///
    /// Tokenize a string
    ///
    /// Parameters
    /// ----------
    /// x : bool
    ///    the string to tokenize
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
    fn get_params(&self) -> PyResult<VTextTokenizerParams> {
        Ok(self.inner.params.clone())
    }

    pub fn __getstate__(&self, py: Python) -> PyResult<PyObject> {
        serialize_params(&self.inner.params, py)
    }

    pub fn __setstate__(&mut self, py: Python, state: PyObject) -> PyResult<()> {
        let mut params: VTextTokenizerParams = deserialize_params(py, state)?;
        self.inner = params.build().unwrap();
        Ok(())
    }
}

/// __init__(self, pattern=r'\\b\\w\\w+\\b')
///
/// Tokenize a document using regular expressions
#[pyclass(extends=BaseTokenizer, module="vtext.tokenize")]
pub struct RegexpTokenizer {
    inner: vtext::tokenize::RegexpTokenizer,
}

#[pymethods]
impl RegexpTokenizer {
    #[new]
    #[args(pattern = "\"\\\\b\\\\w\\\\w+\\\\b\"")]
    fn new(pattern: &str) -> (Self, BaseTokenizer) {
        let inner = vtext::tokenize::RegexpTokenizerParams::default()
            .pattern(pattern)
            .build()
            .unwrap();

        (RegexpTokenizer { inner: inner }, BaseTokenizer::new())
    }

    /// tokenize(self, x)
    ///
    /// Tokenize a string
    ///
    /// Parameters
    /// ----------
    /// x : bool
    ///    the string to tokenize
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
    fn get_params(&self) -> PyResult<RegexpTokenizerParams> {
        Ok(self.inner.params.clone())
    }

    pub fn __getstate__(&self, py: Python) -> PyResult<PyObject> {
        serialize_params(&self.inner.params, py)
    }

    pub fn __setstate__(&mut self, py: Python, state: PyObject) -> PyResult<()> {
        let mut params: RegexpTokenizerParams = deserialize_params(py, state)?;
        self.inner = params.build().unwrap();
        Ok(())
    }
}

/// __init__(self, window_size=4)
///
/// Character tokenizer
///
/// Parameters
/// ----------
/// window_size : str, default=4
///   number of consecutive characters included in a token
///
/// Example
/// -------
/// >>> from vtext.tokenize import CharacterTokenizer
/// >>> tokenizer = CharacterTokenizer(window_size=4)
/// >>> tokenizer.tokenize('fox can\'t')
/// ['fox ', 'ox c', 'x ca', ' can', 'can\'', 'an\'t']
///
#[pyclass(extends=BaseTokenizer, module="vtext.tokenize")]
pub struct CharacterTokenizer {
    inner: vtext::tokenize::CharacterTokenizer,
}

#[pymethods]
impl CharacterTokenizer {
    #[new]
    #[args(window_size = 4)]
    fn new(window_size: usize) -> (Self, BaseTokenizer) {
        let inner = vtext::tokenize::CharacterTokenizerParams::default()
            .window_size(window_size)
            .build()
            .unwrap();

        (CharacterTokenizer { inner: inner }, BaseTokenizer::new())
    }

    /// tokenize(self, x)
    ///
    /// Tokenize a string
    ///
    /// Parameters
    /// ----------
    /// x : bool
    ///    the string to tokenize
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
    fn get_params(&self) -> PyResult<CharacterTokenizerParams> {
        Ok(self.inner.params.clone())
    }

    pub fn __getstate__(&self, py: Python) -> PyResult<PyObject> {
        serialize_params(&self.inner.params, py)
    }

    pub fn __setstate__(&mut self, py: Python, state: PyObject) -> PyResult<()> {
        let mut params: CharacterTokenizerParams = deserialize_params(py, state)?;
        self.inner = params.build().unwrap();
        Ok(())
    }
}
