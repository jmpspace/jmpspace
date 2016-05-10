
# CLEAN THIS UP
# https://stackoverflow.com/questions/11399983/makefile-w-protocol-buffer-and-automatic-dependency

all: rust

rust/target/release/libcontracts.rlib: rust/src/ship.rs rust/src/actions.rs rust/src/world.rs
	(cd rust; cargo build; cargo doc)

rust/src/%.rs: ship.proto actions.proto world.proto
	protoc --rust_out rust/src *.proto

rust: rust/target/release/libcontracts.rlib

.PHONY: rust
