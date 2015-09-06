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

func apply_action(sim *C.Sim, a int) {
	C.apply_action(sim, C.int(a))
}

func update_world(sim *C.Sim) {
	C.update_world(sim)
}

func snapshot_world(sim *C.Sim) {
	C.snapshot_world(sim)
}
