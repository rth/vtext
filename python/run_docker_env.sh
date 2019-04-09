#!/bin/sh
docker run --rm -v $PWD/../:/src -it --net=host --entrypoint "/bin/bash" konstin2/pyo3-pack:0.5.0
