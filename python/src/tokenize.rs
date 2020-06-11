// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::types::{PyBytes, PyTuple};
use bincode::{deserialize, serialize};

use vtext::tokenize::*;

#[pyclass]
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
pub struct UnicodeSegmentTokenizer {
    #[pyo3(get, set)]
    pub word_bounds: bool,
    inner: vtext::tokenize::UnicodeSegmentTokenizer,
}

#[pymethods]
impl UnicodeSegmentTokenizer {
    #[new]
    #[args(word_bounds = true)]
    fn new(word_bounds: bool) -> (Self, BaseTokenizer) {
        let tokenizer = vtext::tokenize::UnicodeSegmentTokenizerParams::default()
            .word_bounds(word_bounds)
            .build()
            .unwrap();

        (
            UnicodeSegmentTokenizer {
                word_bounds: word_bounds,
                inner: tokenizer,
            },
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
    fn get_params(&self) -> PyResult<UnicodeSegmentTokenizerParams> {
        Ok(self.inner.params.clone())
    }

    pub fn __getstate__(&self, py: Python) -> PyResult<PyObject> {
        Ok(PyBytes::new(py, &serialize(&self.word_bounds).unwrap()).to_object(py))
    }

    pub fn __setstate__(&mut self, py: Python, state: PyObject) -> PyResult<()> {
        match state.extract::<&PyBytes>(py) {
            Ok(s) => {
                self.word_bounds = deserialize(s.as_bytes()).unwrap();
                Ok(())
            }
            Err(e) => Err(e),
        }
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
#[pyclass(extends=BaseTokenizer)]
pub struct VTextTokenizer {
    pub lang: String,
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

        (
            VTextTokenizer {
                lang: lang.to_string(),
                inner: tokenizer,
            },
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
}

/// __init__(self, pattern=r'\\b\\w\\w+\\b')
///
/// Tokenize a document using regular expressions
#[pyclass(extends=BaseTokenizer)]
pub struct RegexpTokenizer {
    pub pattern: String,
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

        (
            RegexpTokenizer {
                pattern: pattern.to_string(),
                inner: inner,
            },
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
#[pyclass(extends=BaseTokenizer)]
pub struct CharacterTokenizer {
    pub window_size: usize,
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

        (
            CharacterTokenizer {
                window_size: window_size,
                inner: inner,
            },
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
}
