#!/bin/bash
set -eu

dir=$(dirname "$(dirname "$0")")

if ! [ -d "${dir}/target" ]; then
    mkdir "${dir}/target"
fi

name="package"

# Compile our "script" if the binary doesn't already exist
if [ "${1-}" == "-f" ] || ! [ -f "${dir}/target/$name" ]; then
    echo "Compiling $name..."

    rustc -g --edition=2021 -o "${dir}/target/$name" "${dir}/tools/$name.rs"

    if [ "${1-}" == "-f" ]; then
        # remove the force flag when sending the arguments to generate 
        shift
    fi
fi

install=$(realpath "${dir}/target/$name")
$install "${@}"
