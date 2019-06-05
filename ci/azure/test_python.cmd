@echo on

call activate %VIRTUALENV%

mkdir %TMP_FOLDER%
cd %TMP_FOLDER%

set RUST_BACKTRACE=1
pytest -sv --junitxml=%JUNITXML% --showlocals --pyargs vtext
