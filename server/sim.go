package main

/*
#cgo LDFLAGS: -L./lib -lsim
#include "./include/sim.h"
*/
import "C"
import "unsafe"

func hello_sim() {
	C.hello_sim()
}

func build_world() *C.Sim {
	return C.build_world()
}

func connect_client(sim *C.Sim, client int) int {
	return int(C.connect_client(sim, C.int(client)))
}

func apply_action(sim *C.Sim, client int, buf []byte) int {

	cBuffer := C.struct_BufferImpl{
		length: C.size_t(len(buf)),
		buf:    (*C.uint8_t)(&buf[0])}

	return int(C.apply_action(
		sim,
		C.int(client),
		cBuffer))

}

func update_world(sim *C.Sim) int {
	return int(C.update_world(sim))
}

func snapshot_world(sim *C.Sim) []byte {
	cBuffer := C.snapshot_world(sim)
	return C.GoBytes(unsafe.Pointer(cBuffer.buf), C.int(cBuffer.length))
}
