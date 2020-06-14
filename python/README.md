# vtext

This is a Python wrapper for the Rust vtext crate.

This package aims to provide a high performance toolkit for ingesting textual data for
machine learning applications.

### Features

 - Tokenization: Regexp tokenizer, Unicode segmentation + language specific rules
 - Stemming: Snowball (in Python 15-20x faster than NLTK)
 - Token counting: converting token counts to sparse matrices for use
   in machine learning libraries. Similar to `CountVectorizer` and
   `HashingVectorizer` in scikit-learn but will less broad functionality.
 - Levenshtein edit distance; Sørensen-Dice, Jaro, Jaro Winkler string similarities


### Installation

vtext requires Python 3.6+, numpy 1.15+ and can be installed with,

```
pip install vtext
```

### Documentation

Project documentation: [vtext.io/doc/latest/index.html](https://vtext.io/doc/latest/index.html)


## License

vtext is released under the [Apache License, Version 2.0](./LICENSE).
