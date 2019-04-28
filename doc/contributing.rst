Contributing
============

Building from sources
---------------------

Building requires Python 3.5+ as well as Rust nightly >=1.30.0
(due to `rust-numpy <https://github.com/rust-numpy/rust-numpy>`_ and
`pyo3 <https://github.com/PyO3/pyo3>`_ requirements),

To build the Python package, run,

.. code::

    pip install -r requirements.txt
    python3 setup.py develop --user

Docker environment
------------------

The easiest might be to use docker to setup a build environment,

.. code::

    docker build -t vtext-py-env .
    ./run_docker_env.sh
    rustup toolchain add nightly-2019-02-04
    rustup default nightly-2019-02-04-x86_64-unknown-linux-gnu
    python3.7 -m pip install -r /src/python/requirements.txt
    cd /src/python && python3.7 setup.py install
    python3.7 -m pip install pandas conllu
