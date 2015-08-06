extern crate capnpc;
fn main() {
    assert!(::capnpc::compile("schema", &["Ship.capnp"]).is_ok())
}
