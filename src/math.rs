// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

#![allow(non_snake_case)]

use ndarray::Array2;
use sprs::indexing::SpIndex;

#[derive(Debug)]
pub struct CSRArray {
    pub indices: Vec<usize>,
    pub indptr: Vec<usize>,
    pub data: Vec<i32>,
}

impl CSRArray {
    pub fn to_dense(&self) -> Array2<i32> {
        let n_rows = self.indptr.len() - 1;
        let mut n_columns: usize = 0;
        if let Some(i) = self.indices.iter().max() {
            n_columns = *i + 1;
        }
        let mut X = Array2::<i32>::zeros((n_rows, n_columns));
        for (idx_row, start_stop) in self.indptr.windows(2).enumerate() {
            let start = start_stop[0].index();
            let stop = start_stop[1].index();
            let indices = &self.indices[start..stop];
            let data = &self.data[start..stop];
            for (idx_data, idx_col) in indices.iter().enumerate() {
                X[[idx_row, *idx_col]] = data[idx_data];
            }
        }
        X
    }
}

#[cfg(test)]
mod tests {
    use crate::math::CSRArray;

    #[test]
    fn test_csr_to_dense() {
        let X_csr = CSRArray {
            indices: [3, 1, 1, 5].to_vec(),
            indptr: [0, 2, 4].to_vec(),
            data: [1, 2, 3, 4].to_vec(),
        };

        let X = X_csr.to_dense();
        let X2 = array![[0, 2, 0, 1, 0, 0], [0, 3, 0, 0, 0, 4]];
        assert_eq!(X, X2);
    }
}
