#!/bin/bash

set -e

source activate $VIRTUALENV

TEST_CMD="python -m pytest --showlocals --durations=20 --junitxml=$JUNITXML --pyargs"

mkdir -p $TEST_DIR
cd $TEST_DIR

set -x
$TEST_CMD vtext
set +x
