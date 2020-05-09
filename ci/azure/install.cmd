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

@rem Use oldest supported numpy and scipy versions for building wheels
pip install numpy==1.15.0 scipy==1.1.0 pytest>=4.0.0 wheel>=0.31.1 hypothesis

curl -sSf -o rustup-init.exe https://win.rustup.rs
rustup-init.exe -y --default-toolchain nightly-2019-11-01
set PATH=%PATH%;%USERPROFILE%\.cargo\bin
echo "##vso[task.setvariable variable=PATH;]%PATH%;%USERPROFILE%\.cargo\bin"

rustup default nightly-2019-11-01

call "C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Enterprise\\VC\\Auxiliary\\Build\\vcvars64.bat"

@rem Install the build and runtime dependencies of the project.
cd python/
pip install -r requirements.txt
python setup.py bdist_wheel

pip install pytest-faulthandler

@rem Install the generated wheel package to test it
pip install --pre --no-index --find-links dist\ vtext

if %errorlevel% neq 0 exit /b %errorlevel%
