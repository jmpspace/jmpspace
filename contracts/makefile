
all: src/*.proto
	mkdir -p rust/src
	protoc --rust_out rust/src src/*
	(cd rust; cargo build --release)
	mkdir -p go
	protoc --go_out go src/*
