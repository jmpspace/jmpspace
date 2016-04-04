#!/bin/sh

# cross platform... python?

INPUT_BASE=../contracts
OUTPUT_BASE=./src/contracts

SPACE_SERVER_INPUT=${INPUT_BASE}/spaceServer
SPACE_SERVER_OUTPUT=${OUTPUT_BASE}/spaceServer

protoc --rust_out ${SPACE_SERVER_OUTPUT} --proto_path ${SPACE_SERVER_INPUT} ${SPACE_SERVER_INPUT}/*.proto
