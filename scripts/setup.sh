#!/bin/bash

# set up for tests
python3 -m venv ./tests/venv
. ./tests/venv/bin/activate
pip install --requirement ./tests/requirements.txt
