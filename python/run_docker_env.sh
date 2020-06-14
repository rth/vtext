#!/bin/sh
docker run --rm -v $PWD/../:/src -it --entrypoint "/bin/bash" konstin2/maturin:v0.8.2-alpha.1
