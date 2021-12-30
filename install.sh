#!/bin/bash

PREFIX=""

if [ $# -eq 0 ]; then
    PREFIX=/opt/cartesi/
else
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                echo "This script installs machine manager to the provided prefix (default /opt/cartesi)"
                echo "Usage: ./install.sh [prefix]"
                exit 1
                ;;
            *)
                PREFIX=$1
                shift
                ;;
        esac
    done
fi

echo "Installing to $PREFIX"

mkdir -p $PREFIX/bin/src
mkdir -p $PREFIX/bin/proto
cp ./src/*.py $PREFIX/bin/src
cp ./proto/*.py $PREFIX/bin/proto
cp ./*.py $PREFIX/bin
cp ./machine-manager $PREFIX/bin/machine-manager

echo "Installed"
