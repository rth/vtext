// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

use crate::utils::{deserialize_params, serialize_params};
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyDict;

/// __init__(self, lang='english')
///
/// Snowball stemmer
///
/// Wraps the rust-stemmers crate that uses an implementation generated
/// by the `Snowball compiler <https://github.com/snowballstem/snowball>`_
/// for Rust.
#[pyclass(module = "vtext.stem")]
pub struct SnowballStemmer {
    pub lang: String,
    inner: rust_stemmers::Stemmer,
}

fn get_algorithm(lang: &str) -> PyResult<rust_stemmers::Algorithm> {
    match lang {
        "arabic" => Ok(rust_stemmers::Algorithm::Arabic),
        "danish" => Ok(rust_stemmers::Algorithm::Danish),
        "dutch" => Ok(rust_stemmers::Algorithm::Dutch),
        "english" => Ok(rust_stemmers::Algorithm::English),
        "french" => Ok(rust_stemmers::Algorithm::French),
        "german" => Ok(rust_stemmers::Algorithm::German),
        "greek" => Ok(rust_stemmers::Algorithm::Greek),
        "hungarian" => Ok(rust_stemmers::Algorithm::Hungarian),
        "italian" => Ok(rust_stemmers::Algorithm::Italian),
        "portuguese" => Ok(rust_stemmers::Algorithm::Portuguese),
        "romanian" => Ok(rust_stemmers::Algorithm::Romanian),
        "russian" => Ok(rust_stemmers::Algorithm::Russian),
        "spanish" => Ok(rust_stemmers::Algorithm::Spanish),
        "swedish" => Ok(rust_stemmers::Algorithm::Swedish),
        "tamil" => Ok(rust_stemmers::Algorithm::Tamil),
        "turkish" => Ok(rust_stemmers::Algorithm::Turkish),
        _ => Err(exceptions::ValueError::py_err(format!(
            "lang={} is unsupported!",
            lang
        ))),
    }
}

#[pymethods]
impl SnowballStemmer {
    #[new]
    #[args(lang = "\"english\"")]
    fn new(lang: &str) -> PyResult<Self> {
        let algorithm = get_algorithm(lang)?;
        let stemmer = rust_stemmers::Stemmer::create(algorithm);

        Ok(SnowballStemmer {
            lang: lang.to_string(),
            inner: stemmer,
        })
    }

    /// stem(self, word)
    ///
    /// Stem a string
    ///
    /// Parameters
    /// ----------
    /// word : str
    ///    the string to tokenize
    ///
    /// Returns
    /// -------
    /// word_stemmed : str
    ///      stemmed word
    fn stem(&self, word: &str) -> PyResult<String> {
        let res = self.inner.stem(word).to_string();
        Ok(res)
    }

    fn get_params<'p>(&self, py: Python<'p>) -> PyResult<&'p PyDict> {
        let params = PyDict::new(py);
        params.set_item("lang", self.lang.clone())?;
        Ok(params)
    }

    pub fn __getstate__(&self, py: Python) -> PyResult<PyObject> {
        serialize_params(&self.lang, py)
    }

    pub fn __setstate__(&mut self, py: Python, state: PyObject) -> PyResult<()> {
        self.lang = deserialize_params(py, state)?;
        let algorithm = get_algorithm(&self.lang)?;
        self.inner = rust_stemmers::Stemmer::create(algorithm);
        Ok(())
    }
}
