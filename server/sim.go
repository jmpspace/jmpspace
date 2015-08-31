package main

/*
#cgo LDFLAGS: -L./lib -lsim
#include "./include/sim.h"
*/
import "C"

func hello_sim() {
	C.hello_sim()
}

func build_world(a int) *C.Sim {
	return C.build_world(C.int(a))
}

func apply_command(sim *C.Sim, a int) {
	C.apply_command(sim, C.int(a))
}

func world_snapshot(sim *C.Sim) {
	C.world_snapshot(sim)
}
