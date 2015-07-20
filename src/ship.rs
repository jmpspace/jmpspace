
use tagtree;

enum Part {
  Vessel { width: i32, length: i32 },
  FuelTank { width: i32, length: i32 },
  Engine { width: i32, length: i32, group: i32 }
}

struct Beam {
    length: i32
}

struct Attach {
    location: i32,
    rotation: f64
}

type Structure = tagtree::TagTree<Part,Beam,Attach>;
