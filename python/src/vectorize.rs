// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

use pyo3::prelude::*;
use pyo3::types::PyIterator;

use ndarray::arr1;
use numpy::{IntoPyArray, PyArray1};
use sprs::CsMat;

type PyCsrArray = (Py<PyArray1<i32>>, Py<PyArray1<i32>>, Py<PyArray1<i32>>);

fn iterable_to_collection(text: PyIterator) -> PyResult<Vec<String>> {
    // This should not be necessary, ideally PyIterator should be converted
    // to a Rust iterator

    let collection: Result<Vec<_>, _> = text
        .map(|doc| doc.and_then(ObjectProtocol::extract::<String>))
        .collect();
    Ok(collection?)
}

fn result_to_csr(py: Python, x: CsMat<i32>) -> PyResult<PyCsrArray> {
    // TODO: 1. use slices directly instead of creating new arrays
    //       2. Possibly avoid casing
    //          https://github.com/rust-ndarray/ndarray/issues/493#issuecomment-424043912
    let indices = arr1(x.indices()).mapv(|elem| elem as i32);
    let indptr = arr1(x.indptr()).mapv(|elem| elem as i32);
    let data = arr1(x.data());

    Ok((
        indices.into_pyarray(py).to_owned(),
        indptr.into_pyarray(py).to_owned(),
        data.into_pyarray(py).to_owned(),
    ))
}

#[pyclass]
pub struct _HashingVectorizerWrapper {
    inner: vtext::vectorize::HashingVectorizer<vtext::tokenize::RegexpTokenizer>,
}

#[pymethods]
impl _HashingVectorizerWrapper {
    #[new]
    #[args(n_jobs = 1)]
    fn new(obj: &PyRawObject, n_jobs: usize) {
        let tokenizer = vtext::tokenize::RegexpTokenizer::default();
        let estimator = vtext::vectorize::HashingVectorizerParams::default()
            .tokenizer(tokenizer.clone())
            .n_jobs(n_jobs)
            .build()
            .unwrap();

        obj.init(_HashingVectorizerWrapper { inner: estimator });
    }

    fn transform(&mut self, py: Python, x: PyObject) -> PyResult<PyCsrArray> {
        let text = PyIterator::from_object(py, &x)?;

        let collection = iterable_to_collection(text)?;

        let x = self.inner.fit_transform(&collection);

        result_to_csr(py, x)
    }
}

#[pyclass]
pub struct _CountVectorizerWrapper {
    inner: vtext::vectorize::CountVectorizer<vtext::tokenize::RegexpTokenizer>,
}

#[pymethods]
impl _CountVectorizerWrapper {
    #[new]
    #[args(n_jobs = 1)]
    fn new(obj: &PyRawObject, n_jobs: usize) {
        let tokenizer = vtext::tokenize::RegexpTokenizer::default();
        let estimator = vtext::vectorize::CountVectorizerParams::default()
           .tokenizer(tokenizer.clone())
           .n_jobs(n_jobs)
           .build()
           .unwrap();
        obj.init(_CountVectorizerWrapper { inner: estimator });
    }

    fn fit(&mut self, py: Python, x: PyObject) -> PyResult<()> {
        let text = PyIterator::from_object(py, &x)?;

        let collection = iterable_to_collection(text)?;

        self.inner.fit(&collection);
        Ok(())
    }

    fn get_n_features(&self, py: Python) -> PyResult<(usize)> {
        let n_features = self.inner.vocabulary.len();
        Ok(n_features)
    }

    fn transform(&mut self, py: Python, x: PyObject) -> PyResult<PyCsrArray> {
        let text = PyIterator::from_object(py, &x)?;

        let collection = iterable_to_collection(text)?;

        let x = self.inner.transform(&collection);

        result_to_csr(py, x)
    }

    fn fit_transform(&mut self, py: Python, x: PyObject) -> PyResult<PyCsrArray> {
        let text = PyIterator::from_object(py, &x)?;

        let collection = iterable_to_collection(text)?;

        let x = self.inner.fit_transform(&collection);

        result_to_csr(py, x)
    }
}
