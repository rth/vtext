# text-vectorize benchmarks

This folder includes benchmark scripts for text-vectorize.

To run the benchmarks download an (adapted) copy of the 20 newsgroup dataset
[here](https://s3-eu-west-1.amazonaws.com/public-sym/20newsgoups.zip), then extract
the contents under `text-vectorize/data/`.

Scikit-learn (Python) benchmark can be run with,
```
python benchmarks/bench_scikit-learn.py
```

To run the equivalent processing with text-vectorize (Rust), run,
```
cargo run --release
```
