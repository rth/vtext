use *;

#[test]
fn hashing_vectorizer_init() {
    let documents = vec![String::from("sky is blue"), String::from("sea is blue")];

    let vect = HashingVectorizer::new();
    let vect = vect.fit(&documents);
    let X = vect.transform(&documents);
    println!("{:?}", X);

    //let res2 = vect.fit_transform(&X);
    //assert_eq!(res, res2);
}
