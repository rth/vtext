# Copyright 2019 vtext developers
#
# Licensed under the Apache License, Version 2.0,
# <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
# modified, or distributed except according to those terms.

from setuptools import find_packages, setup
from setuptools_rust import RustExtension


with open("./requirements.txt", "rt") as fh:
    install_requires = fh.read().splitlines()

setup(
    version="0.1.0a3",
    rust_extensions=[
        RustExtension(
            "vtext._lib",
            "./Cargo.toml",
            rustc_flags=["--cfg=Py_3"],
            features=["numpy/python3"],
            args=["--no-default-features"],
        )
    ],
    long_description_content_type="text/markdown",
    url="https://github.com/rth/vtext",
    install_requires=install_requires,
    packages=find_packages(),
)
