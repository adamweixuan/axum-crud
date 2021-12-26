#!/usr/bin/env bash

cargo install cargo-make

cargo make --makefile makefile.toml release
