ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

all: server

server: *.go
	gofmt -w *go
	go get
	go build -ldflags="-r $(ROOT_DIR)lib"

