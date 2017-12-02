#!/usr/bin/env sh

xargo clean
xargo update
unset CARGO_INCREMENTAL
xargo build --example hello --target thumbv7m-none-eabi