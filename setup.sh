#!/bin/bash

rm -rf bin include lib pyvenv.cfg
python3 -m venv .
source bin/activate

pip3 install -r requirements.txt
