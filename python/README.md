# py-text-vectorize

This is a Python wrapper for the Rust text-vectorize crate.

## Installation

### Manual install

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
docker build -t py-text-vectorize-env .
./start_docker_env.sh
python3 setup.py develop --user
```
