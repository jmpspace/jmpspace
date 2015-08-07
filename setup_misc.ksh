#!/bin/ksh

set -u -e

echo "Installing protobuf compiler"
apt-get install protobuf-compiler
ls "$(which protoc)"

echo "Installing go protobuf plugin"
go get -u github.com/golang/protobuf/{proto,protoc-gen-go}
ls "$(which protoc-gen-go)"

echo "Installing rust protobuf plugin"
if [ ! -d rust-protobuf ]
then
  git clone git@github.com:stepancheg/rust-protobuf.git -b master
fi
(cd rust-protobuf; cargo build --release)
cp "./rust-protobuf/target/release/protoc-gen-rust" "${HOME}/bin/"
ls "$(which protoc-gen-rust)"
