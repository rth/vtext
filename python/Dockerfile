FROM rustlang/rust:nightly

COPY requirements.txt .

RUN apt-get update && apt-get install -y python3-pip 
RUN pip3 install -r requirements.txt && \
    pip3 install pytest flake8
WORKDIR /src/
    
