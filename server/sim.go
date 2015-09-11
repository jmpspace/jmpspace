package main

/*
#cgo LDFLAGS: -L./lib -lsim
#include "./include/sim.h"
*/
import "C"

func hello_sim() {
	C.hello_sim()
}

func build_world() *C.Sim {
	return C.build_world()
}

func apply_action(sim *C.Sim, buf []byte) int {
	return int(C.apply_action(sim))
}

func update_world(sim *C.Sim) {
	C.update_world(sim)
}

func snapshot_world(sim *C.Sim) {
	C.snapshot_world(sim)
}
