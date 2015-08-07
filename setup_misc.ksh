#!/bin/ksh

set -u -e -x

echo "Installing protobuf compiler"
if [ ! -d protobuf-2.6.1 ]
then
  wget https://github.com/google/protobuf/releases/download/v2.6.1/protobuf-2.6.1.tar.gz
  tar xvfz protobuf-2.6.1.tar.gz
fi
if ! which protoc
then
  (
  cd protobuf-2.6.1
  ./configure
  make
  make check
  make install
  )
fi
which protoc

echo "Installing go protobuf plugin"
if ! which protoc-gen-go
then
  go get -u github.com/golang/protobuf/{proto,protoc-gen-go}
fi
which protoc-gen-go

echo "Installing rust protobuf plugin"
if [ ! -d rust-protobuf ]
then
  git clone git@github.com:stepancheg/rust-protobuf.git -b master
fi
if ! which protoc-gen-rust
then
  (cd rust-protobuf; cargo build --release)
  cp "./rust-protobuf/target/release/protoc-gen-rust" "${HOME}/bin/"
fi
which protoc-gen-rust

echo "SUCCESS"
