#!/usr/bin/env bash

find src | grep \.rs$ | xargs rustfmt

find src | grep \.cpp$ | xargs clang-format -i

python3 -m isort --profile black .
python3 -m black .