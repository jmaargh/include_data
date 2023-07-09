#!/bin/bash

# This script simply generates the binary test files, consisting of $1
# bytes incrementing from zero

set -euo pipefail

MAX_VAL=$(($1 - 1))
FILE_NAME="binary_${1}"

# Generate a binary file of $1 incrementing bytes
printf '%02x ' $(seq 0 1 ${MAX_VAL}) | xxd -r -p - > "${FILE_NAME}"

echo "Generated $(realpath ${FILE_NAME})"
