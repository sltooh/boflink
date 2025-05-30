#!/bin/sh
# Assumes that the program was installed using `cargo xtask install`.

libexec="~/.local/libexec/boflink"

command="x86_64-w64-mingw32-gcc -B $libexec -fno-lto -nostartfiles -o basic.bof basic.c"
echo $command
eval $command
