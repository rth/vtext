// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

use pyo3::prelude::*;
use pyo3::types::{PyIterator, PyList, PyString};
use pyo3::PyIterProtocol;

use crate::utils::{deserialize_params, serialize_params};
use vtext::token_processing::*;

/// __init__(self, min_n: int, max_n: int, max_k: int)
///
/// K-Skip-N-Grams generator
///
/// Provided with a list of tokens it generates k-skip-n-grams.
///
/// Parameters
/// ----------
/// min_n : int
///    The minimum degree of the ngram
/// max_n : int
///    The maximum degree of the ngram
/// max_k : int
///    The maximum-degree of the skipgram: the total max skip between items
#[pyclass(module = "vtext.token_processing")]
pub struct KSkipNGrams {
    inner: vtext::token_processing::KSkipNGrams,
}

#[pymethods]
impl KSkipNGrams {
    #[new]
    fn new(min_n: usize, max_n: usize, max_k: usize) -> PyResult<Self> {
        let kskipngrams = vtext::token_processing::KSkipNGrams::new(min_n, max_n, max_k);
        Ok(KSkipNGrams { inner: kskipngrams })
    }

    /// transform(self, items: List[str],
    ///     pad_left: Optional[str]=None, pad_right: Optional[str]=None) -> List[List[str]]
    ///
    /// Transforms a given sequence of `items` into k-skip-n-grams.
    ///
    /// Parameters
    /// ----------
    /// items : List[str]
    ///   The list of items to create the k-skip-n-grams of.
    /// pad_left : Optional[str]
    ///   Optional string to use as left padding
    /// pad_right : Optional[str]
    ///   Optional string to use as right padding
    ///
    /// Returns
    /// -------
    /// k-skip-n-grams : List[List[str]]
    ///    computed k-skip-n-grams
    #[args(pad_left = "None", pad_right = "None")]
    fn transform<'py>(
        &self,
        py: Python<'py>,
        items: Vec<&str>,
        pad_left: Option<&str>,
        pad_right: Option<&str>,
    ) -> PyResult<&'py PyList> {
        let res: Vec<_> = self
            .inner
            .transform(Box::new(items.into_iter()), pad_left, pad_right)?
            .collect();
        let output = PyList::new(py, res);
        Ok(output)
    }

    /// get_params(self, x)
    ///
    /// Get parameters for this estimator.
    ///
    /// Returns
    /// -------
    /// params : mapping of string to any
    ///          Parameter names mapped to their values.
    fn get_params(&self) -> PyResult<KSkipNGramsParams> {
        Ok(self.inner.params.clone())
    }

    pub fn __getstate__(&self, py: Python) -> PyResult<PyObject> {
        serialize_params(&self.inner.params, py)
    }

    pub fn __setstate__(&mut self, py: Python, state: PyObject) -> PyResult<()> {
        let mut params: KSkipNGramsParams = deserialize_params(py, state)?;
        self.inner = params.build();
        Ok(())
    }
}
