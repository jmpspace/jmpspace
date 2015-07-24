extern crate capnpc;
fn main() {
    assert!(::capnpc::compile("schema", &["schema/Ship.capnp"]).is_ok())
}
