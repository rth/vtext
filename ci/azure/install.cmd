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
conda create -n %VIRTUALENV% -q -y python=%PYTHON_VERSION% numpy==1.15.0 scipy==1.1.0 pytest==4.3.1 wheel==0.33.1

call activate %VIRTUALENV%

python --version
pip --version

curl -sSf -o rustup-init.exe https://win.rustup.rs
rustup-init.exe -y --default-toolchain nightly-2019-02-28
set PATH=%PATH%;%USERPROFILE%\.cargo\bin
echo "##vso[task.setvariable variable=PATH;]%PATH%;%USERPROFILE%\.cargo\bin"
rustup target add x86_64-unknown-linux-musl

@rem Install the build and runtime dependencies of the project.
cd python/
pip install -r requirements.txt
python setup.py bdist_wheel

@rem Install the generated wheel package to test it
pip install --pre --no-index --find-links dist\ vtextpy

if %errorlevel% neq 0 exit /b %errorlevel%
