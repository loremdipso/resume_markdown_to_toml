#!/bin/bash

cd "${0%/*}/../dist"
google-chrome --headless --no-pdf-header-footer --print-to-pdf=temp.pdf resume.html
ps2pdf temp.pdf resume.pdf
rm temp.pdf
