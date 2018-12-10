#!/bin/sh
docker run --rm -v $PWD/../:/src -it rthz/rust-nightly-python37 /bin/bash
