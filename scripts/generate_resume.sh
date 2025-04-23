#!/bin/bash

cd "${0%/*}/..dist"
trunk build
# ./scripts/export.sh

google-chrome --headless --no-pdf-header-footer --print-to-pdf=temp.pdf index.html
ps2pdf temp.pdf resume.pdf
rm temp.pdf
cp -f resume.pdf resume.pdf
