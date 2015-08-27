package main

// #include <sim.h>
import "C"

func hello_sim() {
	C.hello_sim()
}
