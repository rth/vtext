from glob import glob

from sklearn.feature_extraction.text import HashingVectorizer
from sklearn.feature_extraction.text import CountVectorizer

if __name__ == '__main__':
    input_files = list(glob('./data/*/*'))
    vect = CountVectorizer(input="filename", lowercase=False)
    vect.fit_transform(input_files)
