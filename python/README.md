# vtextpy

This is a Python wrapper for the Rust vtext crate.

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
docker build -t vtextpy-env .
./run_docker_env.sh
rustup toolchain add nightly-2019-02-04
rustup default nightly-2019-02-04-x86_64-unknown-linux-gnu
python3.7 -m pip install -r /src/python/requirements.txt
cd /src/python && python3.7 setup.py install
python3.7 -m pip install pandas conllu
```
