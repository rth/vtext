use ::*;

#[test]
fn hashing_vectorizer_init() {

    let vect = HashingVectorizer::new();

    let X = vec![String::from("sky is blue"),
                  String::from("sea is blue")];

    let vect = vect.fit(&X);
    let res = vect.transform(&X);
    assert_eq!(res, 6);

    let res2 = vect.fit_transform(&X);
    assert_eq!(res, res2);
}
