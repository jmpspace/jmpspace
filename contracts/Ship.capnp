@0xd9391b243c2f5588;

using Go = import "go.capnp";

$Go.package("jmpspace");

struct Part {
  union {
    vessel :group {
      width @0 :Float64;
      length @1 :Float64;
    }
    fuelTank :group {
      radius @2 :Float64;
      length @3 :Float64;
    }
  }
}
