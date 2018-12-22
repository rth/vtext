#![feature(specialization)]

#[macro_use]
extern crate ndarray;
extern crate numpy;
extern crate pyo3;
extern crate text_vectorize;

use ndarray::arr1;
use numpy::{IntoPyArray, PyArray1};
use pyo3::prelude::{pymodinit, ObjectProtocol, Py, PyModule, PyObject, PyResult, Python};
use pyo3::types::PyIterator;
use text_vectorize::HashingVectorizer;

use text_vectorize::tokenize;

fn vec_usize_to_i32(vec: Vec<usize>) -> Vec<i32> {
    let mut vect_out: Vec<i32> = Vec::new();
    for element in vec.iter() {
        if *element > std::i32::MAX as usize {
            panic!("Cannot safely coerce indices to i32!");
        } else {
            vect_out.push(*element as i32);
        }
    }
    vect_out
}

#[pymodinit]
fn _lib(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "hash_vectorize")]
    fn hash_vectorize(
        py: Python,
        x: PyObject,
    ) -> PyResult<(Py<PyArray1<i32>>, Py<PyArray1<i32>>, Py<PyArray1<i32>>)> {
        let text = PyIterator::from_object(py, &x)?;

        let mut collection: Vec<String> = Vec::new();

        for document in text {
            let document = document?;
            let document = ObjectProtocol::extract::<String>(document)?;
            collection.push(document);
        }

        let mut vect = HashingVectorizer::new();
        let x = vect.fit_transform(&collection);

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

    Ok(())
}
