set -e

source activate $VIRTUALENV

pip install awscli

aws s3 sync --cache-control max-age=60 python/dist/ s3://vtext/dist/
