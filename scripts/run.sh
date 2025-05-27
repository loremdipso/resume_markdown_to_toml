#!/usr/bin/bash
set -e

cd "${0%/*}/.."
cargo run -- --input ./test/resume.md --output ./dist/resume.html
cd dist
google-chrome --new-tab 127.0.0.1:8000/resume.html
python3 -m http.server -d .
