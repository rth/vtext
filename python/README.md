# vtext

This is a Python wrapper for the Rust vtext crate.

This package aims to provide a high performance toolkit for ingesting textual data for
machine learning applications.

The API is currently unstable.

### Features

 - Tokenization: Regexp tokenizer, Unicode segmentation + language specific rules
 - Stemming: Snowball (in Python 15-20x faster than NLTK)
 - Token counting: converting token counts to sparse matrices for use
   in machine learning libraries. Similar to `CountVectorizer` and
   `HashingVectorizer` in scikit-learn.
 - Levenshtein edit distance; Sørensen-Dice, Jaro, Jaro Winkler string similarities


### Installation

vtext requires Python 3.5+ and can be installed with,

```
pip install --pre vtext
```

### Documentation

Project documentation: [vtext.io/doc/latest/index.html](https://vtext.io/doc/latest/index.html)


## License

vtext is released under the [Apache License, Version 2.0](./LICENSE).
