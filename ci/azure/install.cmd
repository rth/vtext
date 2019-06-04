@rem https://github.com/numba/numba/blob/master/buildscripts/incremental/setup_conda_environment.cmd
@rem The cmd /C hack circumvents a regression where conda installs a conda.bat
@rem script in non-root environments.
set CONDA_INSTALL=cmd /C conda install -q -y
set PIP_INSTALL=pip install -q

@echo on

@rem Deactivate any environment
call deactivate
@rem Clean up any left-over from a previous build
conda remove --all -q -y -n %VIRTUALENV%
conda create -n %VIRTUALENV% -q -y python=%PYTHON_VERSION%

call activate %VIRTUALENV%

python --version
pip --version

pip install numpy>=1.12.0 scipy>=1.0.0 pytest>=4.0.0 wheel>=0.31.1

curl -sSf -o rustup-init.exe https://win.rustup.rs
rustup-init.exe -y --default-toolchain nightly-2019-02-28
set PATH=%PATH%;%USERPROFILE%\.cargo\bin
echo "##vso[task.setvariable variable=PATH;]%PATH%;%USERPROFILE%\.cargo\bin"

rustup default nightly-2019-02-28

@rem Install the build and runtime dependencies of the project.
cd python/
pip install -r requirements.txt
python setup.py bdist_wheel

pip install pytest-faulthandler

@rem Install the generated wheel package to test it
pip install --pre --no-index --find-links dist\ vtext

if %errorlevel% neq 0 exit /b %errorlevel%
