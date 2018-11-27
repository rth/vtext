#![feature(specialization)]

#[macro_use]
extern crate pyo3;
extern crate ndarray;
extern crate numpy;
extern crate text_vectorize;

use numpy::{IntoPyArray, PyArrayDyn};
use pyo3::prelude::{pymodinit, Py, PyModule, PyResult, Python};

use text_vectorize::tokenize;

#[pymodinit]
fn _lib(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "count_vectorize")]
    fn count_vectorize(
        text: Py<PyArrayDyn<i32>>,
    ) -> (
        Py<PyArrayDyn<i32>>,
        Py<PyArrayDyn<i32>>,
        Py<PyArrayDyn<i32>>,
    ) {
        let indices: Vec<i32> = Vec::new();
        let indptr: Vec<i32> = Vec::new();
        let values: Vec<i32> = Vec::new();
        indices.push(0);
        indptr.push(1);
        values.push(1);

        // XXX: add actual call to text_vectorize

        (
            indices.into_pyarray(_py).to_owned(),
            values.into_pyarray(_py).to_owned(),
            indptr.into_pyarray(_py).to_owned(),
        )
    }

    Ok(())
}
