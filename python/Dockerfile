FROM rustlang/rust:nightly

COPY requirements.txt .

# Install Python 3.7
RUN wget https://www.python.org/ftp/python/3.7.1/Python-3.7.1.tgz && \
    tar xzf Python-3.7.1.tgz && \
    cd Python-3.7.1 && \
    ./configure --enable-optimizations && \
    make build_all -j3 && \
    make altinstall && \
    update-alternatives --install /usr/bin/python python /usr/local/bin/python3.7 50 && \
    cd .. &&  rm -rf Python-3.7.1*

# Install Python 3.6 (while 3.5 is was installed on the system)
# We install different versions of Python to build wheels
RUN wget https://www.python.org/ftp/python/3.6.7/Python-3.6.7.tgz && \
    tar xzf Python-3.6.7.tgz && \
    cd Python-3.6.7 && \
    ./configure --enable-optimizations && \
    make build_all -j3 && \
    make altinstall && \
    cd .. &&  rm -rf Python-3.6.7*

WORKDIR /src/python
    
