use ndarray::Array2;
use sprs::indexing::SpIndex;

#[derive(Debug)]
pub struct CSRArray {
    pub indices: Vec<usize>,
    pub indptr: Vec<usize>,
    pub data: Vec<i32>,
}

fn sort_indices_data_slices<N: Copy, I: SpIndex>(
    // Copied from sprs, as it is not exposed publicly
    indices: &mut [I],
    data: &mut [N],
    buf: &mut Vec<(I, N)>,
) {
    let len = indices.len();
    assert_eq!(len, data.len());
    let indices = &mut indices[..len];
    let data = &mut data[..len];
    buf.clear();
    buf.reserve_exact(len);
    for i in 0..len {
        buf.push((indices[i], data[i]));
    }

    buf.sort_by_key(|x| x.0);

    for (i, &(ind, x)) in buf.iter().enumerate() {
        indices[i] = ind;
        data[i] = x;
    }
}

impl CSRArray {
    pub fn sort_indices(&mut self) {
        // Sort indices for a CSR array inplace
        let mut buf = Vec::new();
        for start_stop in self.indptr.windows(2) {
            let start = start_stop[0].index();
            let stop = start_stop[1].index();
            let indices = &mut self.indices[start..stop];
            let data = &mut self.data[start..stop];
            let len = stop - start;
            let indices = &mut indices[..len];
            let data = &mut data[..len];
            sort_indices_data_slices(indices, data, &mut buf);
        }
    }

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
    fn test_csr_sort_indices() {
        let mut X = CSRArray {
            indices: [3, 1, 1, 5].to_vec(),
            indptr: [0, 2, 4].to_vec(),
            data: [1, 2, 3, 4].to_vec(),
        };
        X.sort_indices();
        let X_ref = CSRArray {
            indices: [1, 3, 1, 5].to_vec(),
            indptr: [0, 2, 4].to_vec(),
            data: [2, 1, 3, 4].to_vec(),
        };

        assert_eq!(X.indices, X_ref.indices);
        assert_eq!(X.indptr, X_ref.indptr);
        assert_eq!(X.data, X_ref.data);
    }

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
