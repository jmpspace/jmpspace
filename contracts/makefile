
all: rust go

rust/target/release/libcontracts.rlib: rust/src/*.rs
	(cd rust; cargo build --release)

rust/src/%.rs: src/%.proto
	protoc --rust_out rust/src src/*

rust: rust/target/release/libcontracts.rlib

go/src/%.pb.go: src/%.proto
	protoc --go_out go src/*

go: go/src/*.pb.go

.PHONY: rust go
