#!/bin/bash

for rust_file in ./src/bin/*.rs; do
    rustfmt $rust_file
done