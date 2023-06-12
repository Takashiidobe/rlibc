#!/bin/bash

set -euo pipefail

# clean up build folder
rm -rf build

# generate folders if required
mkdir -p c build

# build the shlib
cargo b

# Generate the cbindgen
cbindgen --lang=c --output=c/header.h

# Compile the code
gcc -O2 -o build/example c/main.c -L target/debug -lrlibc -Wno-builtin-declaration-mismatch

# Run it with linked dylib
LD_LIBRARY_PATH=target/debug ./build/example

# Print tests passed
echo "OK"

# cleanup, remove example
rm build/example
