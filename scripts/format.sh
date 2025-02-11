#!/bin/bash

for rust_file in ./*.rs; do
    rustfmt $rust_file
done