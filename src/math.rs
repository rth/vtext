extern crate sprs;

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

pub fn sort_indices(X: &mut CSRArray) {
    // Sort indices for a CSR array inplace
    let mut buf = Vec::new();
    for start_stop in X.indptr.windows(2) {
        let start = start_stop[0].index();
        let stop = start_stop[1].index();
        let indices = &mut X.indices[start..stop];
        let data = &mut X.data[start..stop];
        let len = stop - start;
        let indices = &mut indices[..len];
        let data = &mut data[..len];
        sort_indices_data_slices(indices, data, &mut buf);
    }
}

#[cfg(test)]
mod tests {
    use math::{sort_indices, CSRArray};

    #[test]
    fn test_csr_sort_indices() {
        let mut X = CSRArray {
            indices: [3, 1, 1, 5].to_vec(),
            indptr: [0, 2, 4].to_vec(),
            data: [1, 2, 3, 4].to_vec(),
        };
        println!("Before {:?}", X);

        sort_indices(&mut X);
        println!("After {:?}", X);
        let X_ref = CSRArray {
            indices: [1, 3, 1, 5].to_vec(),
            indptr: [0, 2, 4].to_vec(),
            data: [2, 1, 3, 4].to_vec(),
        };

        assert_eq!(X.indices, X_ref.indices);
        assert_eq!(X.indptr, X_ref.indptr);
        assert_eq!(X.data, X_ref.data);
    }
}
