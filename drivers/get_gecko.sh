#!/usr/bin/env bash

OS="macos"

TAR="https://github.com/mozilla/geckodriver/releases/download/v0.34.0/geckodriver-v0.34.0-${OS}.tar.gz"

curl -s -L $TAR | tar xzv - -C .
