#!/bin/bash

# set up for tests
python3 -m venv ./tests/venv
source ./tests/venv/bin/activate
pip install --requirement ./tests/requirements.txt
