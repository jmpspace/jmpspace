package main

/*
#cgo LDFLAGS: -L./lib -lsim
#include "./include/sim.h"
*/
import "C"

func hello_sim() {
	C.hello_sim()
}
