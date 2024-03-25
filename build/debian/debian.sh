#!/bin/sh

# TODO: check for rust dependency

# create a directory structure that mirrors tha final structure of the installed package
mkdir -p rustmon/DEBIAN
mkdir -p rustmon/usr/bin

# build the executable
cd ../..
cargo build --release
cd build/debian

# copy the executable
cp ../../target/release/rustmon rustmon/usr/bin/

# create the control file
touch rustmon/DEBIAN/control

# edit the control file
echo "Package: rustmon
Version: 1.0.0
Section: base
Priority: optional
Architecture: amd64
Maintainer: Vomitblood <tohyouxuan@gmail.com>
Description: "Pokemon Colorscripts written in Rust"" > rustmon/DEBIAN/control

# actually build the thing
echo $(pwd)
dpkg-deb --build rustmon