#!/usr/bin/env bash

cargo clippy -- -D warnings

clang-tidy $(find src | grep \.cpp$) --