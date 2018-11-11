use *;

extern crate sprs;
use sprs::CsMat;

#[test]
fn hashing_vectorizer_init() {
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

    //let vect = HashingVectorizer::new();
    //let vect = vect.fit(&documents);
    //let X = vect.transform(&documents);
    //println!("{:?}", X);
    //let indptr: Vec<usize> = Vec::from(X.indptr());
    //assert_eq!(indptr, vec![0, 4, 8])
    //
    let indptr = vec![0, 5, 8];
    let indices = vec![761698, 328290, 828689, 761698, 780185, 901149, 780185, 144749, 258307];
    let data = vec![1, 1, 1, 1, 1, 1, 1, 1, 1];
    let a = CsMat::new_csc((2, 1000000), indptr, indices, data);

    //let res2 = vect.fit_transform(&X);
    //assert_eq!(res, res2);
}
