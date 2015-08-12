
# CLEAN THIS UP
# https://stackoverflow.com/questions/11399983/makefile-w-protocol-buffer-and-automatic-dependency

all: rust go

rust/target/release/libcontracts.rlib: rust/src/*.rs
	(cd rust; cargo build --release)

rust/src/%.rs: src/%.proto
	protoc --rust_out rust/src src/*

rust: rust/target/release/libcontracts.rlib

go/src/controls.pb.go: src/controls.proto
	protoc --go_out go $<

go/src/ship.pb.go: src/ship.proto
	protoc --go_out go $<

go: go/src/ship.pb.go go/src/controls.pb.go

.PHONY: rust go
