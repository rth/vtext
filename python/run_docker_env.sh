#!/bin/sh
docker run --rm -v $PWD/../:/src -it --entrypoint "/bin/bash" konstin2/pyo3-pack
