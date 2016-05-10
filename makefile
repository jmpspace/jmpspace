
all: server ws

contracts:
	$(MAKE) -C contracts

lc4:
	$(MAKE) -C lc4

sim: lc4 contracts
	$(MAKE) -C sim

client: contracts
	$(MAKE) -C client

server: sim client contracts
	$(MAKE) -C server

ws: sim client contracts
	$(MAKE) -C ws

run:
	./run.sh

.PHONY: contracts lc4 sim client server run
