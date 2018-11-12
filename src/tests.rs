use *;

#[test]
fn count_vectorizer_simple() {
    let documents = vec![
        String::from("the moon in the sky"),
        String::from("The sky sky sky is blue"),
    ];

    let mut vect = CountVectorizer::new();
    vect.fit(&documents);
    let X = vect.transform(&documents);

    println!("{:?}", X.to_dense());
    assert_eq!(X.to_dense(), array![[0, 1, 0, 1, 1, 2], [1, 0, 1, 0, 3, 1]])
}

#[test]
fn hashing_vectorizer_simple() {
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
    assert_eq!(X.indptr, vec![0, 4, 8]);
    assert_eq!(X.data, vec![1, 2, 1, 1, 1, 1, 1, 1]);
    // this is not a thorough test because indices don't match exactly
    // as hashing is not exactly identical
    assert_eq!(X.data.len(), X.indices.len());

    let mut indices_ref = vec![
        268391, 286878, 720286, 828689, 144749, 268391, 286878, 790269,
    ];
    indices_ref.sort();
    indices_ref.dedup();
    let mut indices = X.indices.to_vec();
    indices.sort();
    indices.dedup();
    assert_eq!(indices_ref.len(), indices.len());

    let X2 = vect.fit_transform(&documents);
    //assert_eq!(X.indices, X2.indices);
    assert_eq!(X.indptr, X2.indptr);
    assert_eq!(X.data, X2.data);
}
