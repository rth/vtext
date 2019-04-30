# adapted from pyo3-pack
FROM quay.io/pypa/manylinux1_x86_64

ENV PATH /root/.cargo/bin:$PATH
# Add all supported python versions
ENV PATH /opt/python/cp35-cp35m/bin/:/opt/python/cp36-cp36m/bin/:/opt/python/cp37-cp37m/bin/:$PATH
# Otherwise `cargo new` errors
ENV USER root

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y \
    && rustup toolchain add nightly-2019-02-06 \
    && rustup default nightly-2019-02-06-x86_64-unknown-linux-gnu \
    && rustup component add rustfmt-preview \
    && mkdir /io \
    && python3 -m pip install cffi

WORKDIR /src/

