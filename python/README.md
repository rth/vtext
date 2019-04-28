# vtext

This is a Python wrapper for the Rust vtext crate.

This package aims to provide a high performance toolkit for ingesting textual data for
machine learning applications.

The API is currently unstable.

### Features

 - Tokenization: Regexp tokenizer, Unicode segmentation
 - Stemming: Snowball (in Python 15-20x faster than NLTK) + language specific rules
 - Analyzers (*planned*): word and character n-grams, skip grams
 - Token counting: converting token counts to sparse matrices for use
   in machine learning libraries. Similar to `CountVectorizer` and
   `HashingVectorizer` in scikit-learn.
 - Feature weighting (*planned*): feature weighting based on document
   frequency (TF-IDF), feature normalization.


### Installation

```
pip install vtext
```


### Building from sources

Building requires Python 3.5+ as well as Rust nightly >=1.30.0
(due to [rust-numpy](https://github.com/rust-numpy/rust-numpy) and
[pyo3](https://github.com/PyO3/pyo3) requirements),

To build the Python package, run,
```
pip install -r requirements.txt
python3 setup.py develop --user
```

### Docker environment

The easiest might be to use docker to setup a build environment,

```
docker build -t vtext-py-env .
./run_docker_env.sh
rustup toolchain add nightly-2019-02-04
rustup default nightly-2019-02-04-x86_64-unknown-linux-gnu
python3.7 -m pip install -r /src/python/requirements.txt
cd /src/python && python3.7 setup.py install
python3.7 -m pip install pandas conllu
```
