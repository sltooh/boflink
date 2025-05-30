#!/bin/sh
# Assumes that the program was installed using `cargo xtask install`.

libexec="~/.local/libexec/boflink"

command="x86_64-w64-mingw32-dlltool -l libmyapi.a -d myapi.def"
echo $command
eval $command

command="x86_64-w64-mingw32-gcc -B $libexec -fno-lto -nostartfiles -Wl,--custom-api=libmyapi.a -o custom-api.bof custom-api.c"
echo $command
eval $command
