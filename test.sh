#!/bin/bash

set -euo pipefail

# generate folders if required
mkdir -p c build

# Generate the cbindgen
cbindgen --lang=c --output=c/header.h

# Compile the code
gcc -o build/example c/main.c c/header.h -L target/debug -lrlibc -Wno-builtin-declaration-mismatch

# Run it with linked dylib
LD_LIBRARY_PATH=target/debug ./build/example

# Print tests passed
echo "OK"

# cleanup, remove example
rm build/example
