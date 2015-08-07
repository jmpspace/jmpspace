
all: src/*.proto
	mkdir -p rust/src
	protoc --rust_out rust/src src/*
	mkdir -p go
	protoc --go_out go src/*

clean:
	rm -rf rust/src
	rm -rf go
