from glob import glob

from sklearn.feature_extraction.text import HashingVectorizer

if __name__ == '__main__':
    input_files = list(glob('./data/*/*'))
    vect = HashingVectorizer(input="filename")
    vect.fit_transform(input_files)
