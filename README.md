# vtext

[![Crates.io](https://img.shields.io/crates/v/vtext.svg)](https://crates.io/crates/vtext)
[![PyPI](https://img.shields.io/pypi/v/vtext.svg)](https://pypi.org/project/vtext/)
[![CircleCI](https://circleci.com/gh/rth/vtext/tree/master.svg?style=svg)](https://circleci.com/gh/rth/vtext/tree/master)
[![Build Status](https://dev.azure.com/ryurchak/vtext/_apis/build/status/rth.vtext?branchName=master)](https://dev.azure.com/ryurchak/vtext/_build/latest?definitionId=1&branchName=master)

NLP in Rust with Python bindings

This package aims to provide a high performance toolkit for ingesting textual data for
machine learning applications.

The API is currently unstable.

### Features

 - Tokenization: Regexp tokenizer, Unicode segmentation + language specific rules
 - Stemming: Snowball (in Python 15-20x faster than NLTK)
 - Token counting: converting token counts to sparse matrices for use
   in machine learning libraries. Similar to `CountVectorizer` and
   `HashingVectorizer` in scikit-learn but will less broad functionality.
 - Levenshtein edit distance; Sørensen-Dice, Jaro, Jaro Winkler string similarities

## Usage

### Usage in Python

vtext requires Python 3.5+ and can be installed with,
```
pip install --pre vtext
```

Below is a simple tokenization example,

```python
>>> from vtext.tokenize import VTextTokenizer
>>> VTextTokenizer("en").tokenize("Flights can't depart after 2:00 pm.")
["Flights", "ca", "n't", "depart" "after", "2:00", "pm", "."]
```

For more details see the project documentation: [vtext.io/doc/latest/index.html](https://vtext.io/doc/latest/index.html)

### Usage in Rust

Add the following to `Cargo.toml`,
```toml
[dependencies]
vtext = "0.1.0-alpha.2"
```

For more details see rust documentation: [docs.rs/vtext](https://docs.rs/vtext)

## Benchmarks

#### Tokenization

Following benchmarks illustrate the tokenization accuracy (F1 score) on [UD treebanks](https://universaldependencies.org/)
,

                    
|  lang | dataset   |regexp    | spacy 2.1 | vtext    |         
|-------|-----------|----------|-----------|----------|
|  en   | EWT       | 0.812    | 0.972     | 0.966    |
|  en   | GUM       | 0.881    | 0.989     | 0.996    |
|  de   | GSD       | 0.896    | 0.944     | 0.964    |
|  fr   | Sequoia   | 0.844    | 0.968     | 0.971    |

and the English tokenization speed,

|                          |regexp | spacy 2.1 | vtext |
|--------------------------|-------|-----------|-------|
| **Speed** (10⁶ tokens/s) | 3.1   | 0.14      | 2.1   |


#### Text vectorization

Below are  benchmarks for converting
textual data to a sparse document-term matrix using the 20 newsgroups dataset, 
run on Intel(R) Xeon(R) CPU E3-1270 v6 @ 3.80GHz,

| Speed (MB/s)                  | scikit-learn 0.20.1 | vtext (n_jobs=1) | vtext (n_jobs=4) |
|-------------------------------|---------------------|------------------|------------------|
| CountVectorizer.fit           |  14                 | 104              | 225              |
| CountVectorizer.transform     |  14                 | 82               | 303              |
| CountVectorizer.fit_transform |  14                 | 70               | NA               |
| HashingVectorizer.transform   |  19                 | 89               | 309              |


Note however that these two estimators in vtext currently support only a fraction of
scikit-learn's functionality.  See [benchmarks/README.md](./benchmarks/README.md)
for more details.


## License

vtext is released under the [Apache License, Version 2.0](./LICENSE).
