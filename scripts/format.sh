#!/usr/bin/env bash

find src | grep \.rs$ | xargs rustfmt