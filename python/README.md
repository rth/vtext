# vtext

This is a Python wrapper for the Rust vtext crate.

This package aims to provide a high performance toolkit for ingesting textual data for
machine learning applications.

The API is currently unstable.

### Features

 - Tokenization: Regexp tokenizer, Unicode segmentation + language specific rules
 - Stemming: Snowball (in Python 15-20x faster than NLTK)
 - Analyzers (*planned*): word and character n-grams, skip grams
 - Token counting: converting token counts to sparse matrices for use
   in machine learning libraries. Similar to `CountVectorizer` and
   `HashingVectorizer` in scikit-learn.
 - Feature weighting (*planned*): feature weighting based on document
   frequency (TF-IDF), feature normalization.
 - String similarity: Sørensen-Dice, Jaro, Jaro Winkler


### Installation

vtext requires Python 3.5+ and can be installed with,

```
pip install --pre vtext
```

### Documentation

Project documentation: [vtext.io/doc/latest/index.html](https://vtext.io/doc/latest/index.html)

## License

vtext is released under the BSD 3-clause license.

