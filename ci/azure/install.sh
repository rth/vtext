#!/bin/bash

set -ex

UNAMESTR=`uname`

TO_INSTALL="python=$PYTHON_VERSION pip"

conda create -n $VIRTUALENV --yes $TO_INSTALL
source activate $VIRTUALENV

python --version
pip --version

# Use oldest supported numpy, scipy versins for building wheels
pip install numpy==1.15.0 scipy==1.1.0 pytest>=4.0.0 wheel>=0.31.1 hypothesis

python -c "import numpy; print('numpy %s' % numpy.__version__)"
python -c "import scipy; print('scipy %s' % scipy.__version__)"
pip list

curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly-2019-11-01
source $HOME/.cargo/env
rustup default nightly-2019-11-01

cd python/
pip install -r requirements.txt
python setup.py bdist_wheel

pip install --pre --no-index --find-links dist/ vtext
