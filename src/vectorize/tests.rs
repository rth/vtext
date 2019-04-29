// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

use crate::vectorize::*;
use crate::*;

#[test]
fn test_count_vectorizer_simple() {
    // Example 1

    let documents = vec![String::from("cat dog cat")];
    let mut vect = CountVectorizer::new();
    let X = vect.fit_transform(&documents);
    assert_eq!(X.to_dense(), array![[2, 1]]);

    // Example 1
    let documents = vec![
        String::from("the moon in the sky"),
        String::from("The sky sky sky is blue"),
    ];

    let mut vect = CountVectorizer::new();
    vect.fit(&documents);
    let X = vect.transform(&documents);

    assert_eq!(X.to_dense().shape(), [2, 6]);
    assert_eq!(X.to_dense(), array![[0, 1, 0, 1, 1, 2], [1, 0, 1, 0, 3, 1]])
}

#[test]
fn test_hashing_vectorizer_simple() {
    // Results with scikit-learn 0.20.0
    // >>> vect = HashingVectorizer(norm=None, alternate_sign=False)
    // >>> X = vect.fit_transform(['the moon in the sky', 'The sky is blue'])
    // >>> X.indices
    // array([268391, 286878, 720286, 828689, 144749, 268391, 286878, 790269],
    //       dtype=int32)
    // >>> X.indptr
    // array([0, 4, 8], dtype=int32)
    // >>> X.data
    // array([1., 2., 1., 1., 1., 1., 1., 1.])
    let documents = vec![
        String::from("the moon in the sky"),
        String::from("The sky is blue"),
    ];

    let vect = HashingVectorizer::new();
    let vect = vect.fit(&documents);
    let X = vect.transform(&documents);
    assert_eq!(X.indptr(), &[0, 4, 8]);
    assert_eq!(X.data(), &[1, 2, 1, 1, 1, 1, 1, 1]);
    // this is not a thorough test because indices don't match exactly
    // as hashing is not exactly identical
    assert_eq!(X.data().len(), X.indices().len());

    let mut indices_ref = vec![
        268391, 286878, 720286, 828689, 144749, 268391, 286878, 790269,
    ];
    indices_ref.sort();
    indices_ref.dedup();
    let mut indices = X.indices().to_vec();
    indices.sort();
    indices.dedup();
    assert_eq!(indices_ref.len(), indices.len());

    let X2 = vect.fit_transform(&documents);
    //assert_eq!(X.indices, X2.indices);
    assert_eq!(X.indptr(), X2.indptr());
    assert_eq!(X.data(), X2.data());
}

#[test]
fn test_empty_dataset() {
    let documents: Vec<String> = vec![];

    let mut vectorizer = CountVectorizer::new();

    let X = vectorizer.fit_transform(&documents);
    assert_eq!(X.data(), &[]);
    assert_eq!(X.indices(), &[]);
    assert_eq!(X.indptr(), &[0]);

    let vectorizer = HashingVectorizer::new();

    let X = vectorizer.fit_transform(&documents);
    assert_eq!(X.data(), &[]);
    assert_eq!(X.indices(), &[]);
    assert_eq!(X.indptr(), &[0]);
}
