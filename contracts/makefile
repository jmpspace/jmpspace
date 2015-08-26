
# CLEAN THIS UP
# https://stackoverflow.com/questions/11399983/makefile-w-protocol-buffer-and-automatic-dependency

all: rust go

rust/target/release/libcontracts.rlib: rust/src/ship.rs rust/src/controls.rs
	(cd rust; cargo build)

rust/src/%.rs: src/ship.proto src/controls.proto
	protoc --rust_out rust/src src/*

rust: rust/target/release/libcontracts.rlib

go/controls/controls.pb.go: src/controls.proto
	cp $< .
	protoc --go_out go/controls controls.proto
	rm controls.proto

go/ship/ship.pb.go: src/ship.proto
	cp $< .
	protoc --go_out go/ship ship.proto
	rm ship.proto

go: go/ship/ship.pb.go go/controls/controls.pb.go

.PHONY: rust go
