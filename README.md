# vtext

![Crates.io](https://img.shields.io/crates/v/vtext.svg)
[![CircleCI](https://circleci.com/gh/rth/vtext/tree/master.svg?style=svg)](https://circleci.com/gh/rth/vtext/tree/master)
[![Build Status](https://dev.azure.com/ryurchak/vtext/_apis/build/status/rth.vtext?branchName=master)](https://dev.azure.com/ryurchak/vtext/_build/latest?definitionId=1&branchName=master)

NLP in Rust with Python bindings

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

## Usage

### Usage in Rust

Add the following to `Cargo.toml`,
```toml
[dependencies]
text-vectorize = "0.1.0-alpha.1"
``` 
A simple tokenization example can be found below,
```rust
extern crate vtext;

use vtext::tokenize::VTextTokenizer;

let tok = VTextTokenizer("en");
let tokens = tok.tokenize("Flights after 2:00 pm can't depart (today and tomorrow).");
// returns &["Flights", "after", "2:00", "pm", "ca", "n't",
//           "depart", "(", "today", "and", "tomorrow", ")", "."]
```
For more details see rust documentation: [docs.rs/vtext](https://docs.rs/vtext)

### Usage in Python

The python package can be installed for Python 3.7 on Linux with,
```
pip install vtext
```
(adjust the URL if you need another python 3 version). Wheels for Windows and Mac OS will be uploaded to PyPi in the near future.

The API aims to be compatible with scikit-learn's
[CountVectorizer](https://scikit-learn.org/stable/modules/generated/sklearn.feature_extraction.text.CountVectorizer.html)
and [HashingVectorizer](https://scikit-learn.org/stable/modules/generated/sklearn.feature_extraction.text.HashingVectorizer.html) 
though only a subset of features will be implemented.


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

and the English tokenization speed in million words per second (MWPS)

|           |regexp    | spacy 2.1 | vtext    | 
|-----------|----------|-----------|----------|
| **Speed** | 3.1 MWPS | 0.14 MWPS | 2.1 MWPS |


#### Text vectorization

Below are  benchmarks for converting
textual data to a sparse document-term matrix using the 20 newsgroups dataset, 

|                     | scikit-learn 0.20.1  | vtext 0.1.0a1    |
|---------------------|----------------------|------------------|
| CountVectorizer     |  14 MB/s             | 35 MB/s          |
| HashingVectorizer   |  19 MB/s             | 68 MB/s          |


see [benchmarks/README.md](./benchmarks/README.md) for more details.


## License

vtext is released under the BSD 3-clause license.
