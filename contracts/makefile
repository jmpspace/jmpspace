
# CLEAN THIS UP
# https://stackoverflow.com/questions/11399983/makefile-w-protocol-buffer-and-automatic-dependency

all: rust go

rust/target/release/libcontracts.rlib: rust/src/ship.rs rust/src/actions.rs rust/src/world.rs
	(cd rust; cargo build; cargo doc)

rust/src/%.rs: src/ship.proto src/actions.proto src/world.proto
	protoc --rust_out rust/src src/*

rust: rust/target/release/libcontracts.rlib

go/actions/actions.pb.go: src/actions.proto
	cp $< .
	protoc --go_out go/actions actions.proto
	rm actions.proto

go/ship/ship.pb.go: src/ship.proto
	cp $< .
	protoc --go_out go/ship ship.proto
	rm ship.proto

go: go/ship/ship.pb.go go/actions/actions.pb.go

.PHONY: rust go
