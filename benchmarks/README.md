# vtext benchmarks

This folder includes run time benchmark scripts for vtext

To run the benchmarks download the following datasets,
  - an (adapted) copy of the 20 newsgroup dataset [here](https://s3-eu-west-1.amazonaws.com/public-sym/20newsgoups.zip), and extract
the contents under `vtext/data/`.
  - the [UD Treebanks v2.3](https://universaldependencies.org/#download) and extract them under `vtext/ud-treebanks-v2.3/`


Various benchmark scrips can then be run in Python. Optional dependencies include,

 - scikit-learn >=0.20
 - nltk
 - spacy
 - python-Levenshtein
 - blingfire

and are used as a performance baseline.
