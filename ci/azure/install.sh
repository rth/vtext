#!/bin/bash

set -e

UNAMESTR=`uname`

make_conda() {
    TO_INSTALL="$@"
    conda create -n $VIRTUALENV --yes $TO_INSTALL
    source activate $VIRTUALENV
}

TO_INSTALL="python=$PYTHON_VERSION pip
	numpy>=1.12.0 scipy>=1.0.0 pytest>=4.0.0 wheel>=0.31.1
	nomkl"

make_conda $TO_INSTALL


python --version
ip --version
python -c "import numpy; print('numpy %s' % numpy.__version__)"
python -c "import scipy; print('scipy %s' % scipy.__version__)"
pip list

curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly-2019-02-28
source $HOME/.cargo/env

cd python/
python setup.py bdist_wheel
