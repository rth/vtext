@echo on

@rem Only 64 bit uses conda
IF "%PYTHON_ARCH%"=="64" (
    call activate %VIRTUALENV%
)

mkdir %TMP_FOLDER%
cd %TMP_FOLDER%

pytest --junitxml=%JUNITXML% --showlocals --durations=20 %PYTEST_ARGS% --pyargs vtext
