#!/bin/sh

# get the script location
SCRIPT_DIR=$(dirname "$0")

# Variables
PKG_NAME=rustmon
BUILD_DIR=$SCRIPT_DIR/build/debian
RELEASE_DIR=$SCRIPT_DIR/../../target/release
DEBIAN_DIR=$BUILD_DIR/$PKG_NAME/DEBIAN
BIN_DIR=$BUILD_DIR/$PKG_NAME/usr/bin
AUTO_INSTALL=${1:-"no"}

# check for rust
if ! command -v rustc &> /dev/null
then
    if [ "$AUTO_INSTALL" = "-y" ]; then
        echo "Rust is not installed. Auto-installing..."
        sudo apt install rustc
        source $HOME/.cargo/env
    else
        echo "Rust is not installed. Would you like to install it now? (yes/no)"
        read answer
        if [ "$answer" != "${answer#[Yy]}" ] ;then
            sudo apt install rustc
            source $HOME/.cargo/env
        else
            echo "Rust is required to continue. Exiting."
            exit 1
        fi
    fi
fi

# create directory structure
mkdir -p $DEBIAN_DIR
mkdir -p $BIN_DIR

# Bbild the executable
cd ../..
cargo build --release
cd $BUILD_DIR

# copy the executable
cp $RELEASE_DIR/$PKG_NAME $BIN_DIR/

# create the control file
touch $DEBIAN_DIR/control

# edit the control file
echo "Package: $PKG_NAME
Version: 1.0.0
Section: base
Priority: optional
Architecture: amd64
Maintainer: Vomitblood <tohyouxuan@gmail.com>
Description: Pokemon Colorscripts written in Rust" > $DEBIAN_DIR/control

# build the package
dpkg-deb --build $PKG_NAME