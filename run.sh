#!/usr/bin/env zsh

rustc $@.rs -o $@.out && ./$@.out && rm $@.out

