// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.
use bincode::{deserialize, serialize};
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn serialize_params<T>(params: &T, py: Python) -> PyResult<PyObject>
where
    T: Serialize,
{
    Ok(PyBytes::new(py, &serialize(&params).unwrap()).to_object(py))
}

pub fn deserialize_params<T>(py: Python, state: PyObject) -> PyResult<T>
where
    T: DeserializeOwned + Clone,
{
    match state.extract::<&PyBytes>(py) {
        Ok(s) => {
            let params: T = deserialize(s.as_bytes()).unwrap();
            Ok(params)
        }
        Err(e) => Err(e),
    }
}
