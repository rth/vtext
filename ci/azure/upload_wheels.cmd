@echo on

call activate $VIRTUALENV

pip install awscli

aws s3 sync python\dist\ s3://vtext/dist/
