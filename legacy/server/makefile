ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

all: server

server: *.go lib/* include/*
	gofmt -w *go
	go get
	go build -gcflags "-N -l" -ldflags "-w -r $(ROOT_DIR)lib"

