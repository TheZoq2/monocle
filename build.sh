#!/usr/bin/env sh

set -e

if [ -z "$1" ];then
    echo "no program-name argument specified"
    echo
    echo "Usage:"
    echo
    echo "./build.sh <program-name>"
    echo
    echo "where: <program-name> = file in the example directory excluding the .rs extention"
    echo "Example: blinky.rs becomes 'blinky' as the program-name"
else
    rm -f elf-image
    xargo clean
    xargo update
    unset CARGO_INCREMENTAL
    xargo build --example "$1" --target thumbv7m-none-eabi
    cp "./target/thumbv7m-none-eabi/debug/examples/$1" elf-image
fi
echo
echo "Press Any key to continue ...."
read