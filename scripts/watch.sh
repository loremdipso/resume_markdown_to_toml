#!/usr/bin/bash

google-chrome --new-tab localhost:8000
python3 -m http.server -d .
