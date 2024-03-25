#!/bin/sh

# TODO: check for rust dependency

# create a directory structure that mirrors tha final structure of the installed package
mkdir -p pokerust/DEBIAN
mkdir -p pokerust/usr/bin

# build the executable
cd ../..
cargo build --release
cd build/debian

# copy the executable
cp ../../target/release/pokerust pokerust/usr/bin/

# create the control file
touch pokerust/DEBIAN/control

# edit the control file
echo "Package: pokerust
Version: 1.0.0
Section: base
Priority: optional
Architecture: amd64
Maintainer: Vomitblood <tohyouxuan@gmail.com>
Description: "Pokemon Colorscripts written in Rust"" > pokerust/DEBIAN/control

# actually build the thing
echo $(pwd)
dpkg-deb --build pokerust