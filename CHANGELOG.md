# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Added `UnicodeSentenceTokenizer` that tokenizes sentences following Unicode segmentation rules using the [`unicode-segmentation`](https://github.com/unicode-rs/unicode-segmentation) crate [#66](https://github.com/rth/vtext/pull/66)
- Added `PunctuationTokenizer` that tokenizes sentences delimited by punctuation [#70](https://github.com/PyO3/pyo3/pull/70)

### Changed
- Updated the Python wrapper to use PyO3 0.10 which in particular raises Rust panics as Python exceptions
  [#69](https://github.com/rth/vtext/pull/69)
- Added Python 3.8 wheel generation [#65](https://github.com/rth/vtext/pull/65)
- Tokenizers can now be pickled in Python [#73](https://github.com/rth/vtext/pull/73)
- Only Python 3.6+ is now supported in the Python package.
- Renamed `UnicodeSegmentTokenizer` to `UnicodeWordTokenizer`.
- Better error handling. In particular `error::VTextError` is replaced by `error::EstimatorErr`.

### Contributors

- Josh Bowles
- Josh Levy-Kramer
- Roman Yurchak
