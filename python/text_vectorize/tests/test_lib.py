from text_vectorize import HashingVectorizer  # noqa


def test_hashing_vectorizer():
    text = ["some sentence", "a different sentence"]
    vect = HashingVectorizer(norm=None)

    X = vect.fit_transform(text)
    assert X.nnz == 4
