#!/bin/sh
# Assumes that the boflink executable exists and is located in the user's PATH.

# Get the path to the 'boflink' executable
boflink=$(which boflink)

command="llvm-dlltool -l libmyapi.a -d myapi.def"
echo $command
eval $command

command="clang --ld-path=$boflink --target=x86_64-windows-gnu -nostartfiles -Wl,--custom-api=libmyapi.a -o custom-api.bof custom-api.c"
echo $command
eval $command
