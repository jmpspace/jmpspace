// This file is generated. Do not edit
// @generated

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;
use super::ship::Vessel;
use super::ship::FuelTank;
use super::ship::Engine;
use super::ship::Part;
use super::ship::Beam;
use super::ship::Root;
use super::ship::Attach;
use super::ship::StructureNode;
use super::ship::StructureLink;
use super::ship::StructureTree;
use super::ship::EndMarker;
use super::ship::StructureData;
use super::ship::Structure;
use super::ship::Ship;

#[derive(Clone,Default)]
pub struct Snapshot {
    // message fields
    ships: ::protobuf::RepeatedField<Ship>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Snapshot {
    pub fn new() -> Snapshot {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Snapshot {
        static mut instance: ::protobuf::lazy::Lazy<Snapshot> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Snapshot,
        };
        unsafe {
            instance.get(|| {
                Snapshot {
                    ships: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .ship.Ship ships = 1;

    pub fn clear_ships(&mut self) {
        self.ships.clear();
    }

    // Param is passed by value, moved
    pub fn set_ships(&mut self, v: ::protobuf::RepeatedField<Ship>) {
        self.ships = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ships<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Ship> {
        &mut self.ships
    }

    // Take field
    pub fn take_ships(&mut self) -> ::protobuf::RepeatedField<Ship> {
        ::std::mem::replace(&mut self.ships, ::protobuf::RepeatedField::new())
    }

    pub fn get_ships<'a>(&'a self) -> &'a [Ship] {
        &self.ships
    }
}

impl ::protobuf::Message for Snapshot {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ships));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.ships.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.ships.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Snapshot>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Snapshot {
    fn new() -> Snapshot {
        Snapshot::new()
    }

    fn descriptor_static(_: ::std::option::Option<Snapshot>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "ships",
                    Snapshot::get_ships,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Snapshot>(
                    "Snapshot",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Snapshot {
    fn clear(&mut self) {
        self.clear_ships();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Snapshot {
    fn eq(&self, other: &Snapshot) -> bool {
        self.ships == other.ships &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Snapshot {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GameState {
    // message fields
    // message oneof groups
    state: ::std::option::Option<GameState_oneof_state>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum GameState_oneof_state {
    focusEntityId(u64),
    snapshot(Snapshot),
}

impl GameState {
    pub fn new() -> GameState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GameState {
        static mut instance: ::protobuf::lazy::Lazy<GameState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GameState,
        };
        unsafe {
            instance.get(|| {
                GameState {
                    state: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 focusEntityId = 1;

    pub fn clear_focusEntityId(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_focusEntityId(&self) -> bool {
        match self.state {
            ::std::option::Option::Some(GameState_oneof_state::focusEntityId(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_focusEntityId(&mut self, v: u64) {
        self.state = ::std::option::Option::Some(GameState_oneof_state::focusEntityId(v))
    }

    pub fn get_focusEntityId<'a>(&self) -> u64 {
        match self.state {
            ::std::option::Option::Some(GameState_oneof_state::focusEntityId(v)) => v,
            _ => 0,
        }
    }

    // optional .world.Snapshot snapshot = 2;

    pub fn clear_snapshot(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_snapshot(&self) -> bool {
        match self.state {
            ::std::option::Option::Some(GameState_oneof_state::snapshot(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_snapshot(&mut self, v: Snapshot) {
        self.state = ::std::option::Option::Some(GameState_oneof_state::snapshot(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_snapshot<'a>(&'a mut self) -> &'a mut Snapshot {
        if let ::std::option::Option::Some(GameState_oneof_state::snapshot(_)) = self.state {
        } else {
            self.state = ::std::option::Option::Some(GameState_oneof_state::snapshot(Snapshot::new()));
        }
        match self.state {
            ::std::option::Option::Some(GameState_oneof_state::snapshot(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_snapshot(&mut self) -> Snapshot {
        if self.has_snapshot() {
            match self.state.take() {
                ::std::option::Option::Some(GameState_oneof_state::snapshot(v)) => v,
                _ => panic!(),
            }
        } else {
            Snapshot::new()
        }
    }

    pub fn get_snapshot<'a>(&'a self) -> &'a Snapshot {
        match self.state {
            ::std::option::Option::Some(GameState_oneof_state::snapshot(ref v)) => v,
            _ => Snapshot::default_instance(),
        }
    }
}

impl ::protobuf::Message for GameState {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.state = ::std::option::Option::Some(GameState_oneof_state::focusEntityId(try!(is.read_uint64())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.state = ::std::option::Option::Some(GameState_oneof_state::snapshot(try!(is.read_message())));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.state {
            match v {
                &GameState_oneof_state::focusEntityId(v) => {
                    my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &GameState_oneof_state::snapshot(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.state {
            match v {
                &GameState_oneof_state::focusEntityId(v) => {
                    try!(os.write_uint64(1, v));
                },
                &GameState_oneof_state::snapshot(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GameState>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GameState {
    fn new() -> GameState {
        GameState::new()
    }

    fn descriptor_static(_: ::std::option::Option<GameState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "focusEntityId",
                    GameState::has_focusEntityId,
                    GameState::get_focusEntityId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "snapshot",
                    GameState::has_snapshot,
                    GameState::get_snapshot,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GameState>(
                    "GameState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GameState {
    fn clear(&mut self) {
        self.clear_focusEntityId();
        self.clear_snapshot();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GameState {
    fn eq(&self, other: &GameState) -> bool {
        self.state == other.state &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GameState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0b, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x05, 0x77,
    0x6f, 0x72, 0x6c, 0x64, 0x1a, 0x0a, 0x73, 0x68, 0x69, 0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0x25, 0x0a, 0x08, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x12, 0x19, 0x0a, 0x05,
    0x73, 0x68, 0x69, 0x70, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x73, 0x68,
    0x69, 0x70, 0x2e, 0x53, 0x68, 0x69, 0x70, 0x22, 0x52, 0x0a, 0x09, 0x47, 0x61, 0x6d, 0x65, 0x53,
    0x74, 0x61, 0x74, 0x65, 0x12, 0x17, 0x0a, 0x0d, 0x66, 0x6f, 0x63, 0x75, 0x73, 0x45, 0x6e, 0x74,
    0x69, 0x74, 0x79, 0x49, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x48, 0x00, 0x12, 0x23, 0x0a,
    0x08, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x0f, 0x2e, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x2e, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74,
    0x48, 0x00, 0x42, 0x07, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x4a, 0x9c, 0x02, 0x0a, 0x06,
    0x12, 0x04, 0x01, 0x00, 0x0e, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x0d,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03, 0x07, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x05, 0x00, 0x07, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x05, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x02, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x06, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x06, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x15, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x06, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x09, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x09, 0x08, 0x11,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x08, 0x00, 0x12, 0x04, 0x0a, 0x02, 0x0d, 0x03, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x08, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x0b, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x0b, 0x0b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x0b, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x04, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0c, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x18, 0x19,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
