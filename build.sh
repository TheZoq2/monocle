#!/usr/bin/env sh

xargo clean
xargo update
unset CARGO_INCREMENTAL
#xargo build --example hello --target thumbv7m-none-eabi
#xargo build --example device --target thumbv7m-none-eabi
xargo build --example blinky --target thumbv7m-none-eabi
echo
echo "Press Any key to continue ...."
read