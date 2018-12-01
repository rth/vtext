FROM rustlang/rust:nightly

COPY requirements.txt .

RUN wget https://www.python.org/ftp/python/3.7.1/Python-3.7.1.tgz && \
    tar xzf Python-3.7.1.tgz && \
    cd Python-3.7.1 && \
    ./configure --enable-optimizations && \
    make build_all -j3 && \
    make altinstall && \
    update-alternatives --install /usr/bin/python python /usr/local/bin/python3.7 50 && \
    cd .. &&  rm -rf Python-3.7.1*
RUN python -m pip install -r requirements.txt && \
    python -m pip install pytest flake8
WORKDIR /src/python
    
