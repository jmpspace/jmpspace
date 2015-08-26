// This file is generated. Do not edit
// @generated

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Vessel {
    // message fields
    width: ::std::option::Option<f64>,
    length: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Vessel {
    pub fn new() -> Vessel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Vessel {
        static mut instance: ::protobuf::lazy::Lazy<Vessel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Vessel,
        };
        unsafe {
            instance.get(|| {
                Vessel {
                    width: ::std::option::Option::None,
                    length: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required double width = 1;

    pub fn clear_width(&mut self) {
        self.width = ::std::option::Option::None;
    }

    pub fn has_width(&self) -> bool {
        self.width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_width(&mut self, v: f64) {
        self.width = ::std::option::Option::Some(v);
    }

    pub fn get_width<'a>(&self) -> f64 {
        self.width.unwrap_or(0.)
    }

    // required double length = 2;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: f64) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length<'a>(&self) -> f64 {
        self.length.unwrap_or(0.)
    }
}

impl ::protobuf::Message for Vessel {
    fn is_initialized(&self) -> bool {
        if self.width.is_none() {
            return false;
        };
        if self.length.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.width = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.length = ::std::option::Option::Some(tmp);
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
        if self.width.is_some() {
            my_size += 9;
        };
        if self.length.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.width {
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.length {
            try!(os.write_double(2, v));
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
        ::std::any::TypeId::of::<Vessel>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Vessel {
    fn new() -> Vessel {
        Vessel::new()
    }

    fn descriptor_static(_: ::std::option::Option<Vessel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "width",
                    Vessel::has_width,
                    Vessel::get_width,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "length",
                    Vessel::has_length,
                    Vessel::get_length,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Vessel>(
                    "Vessel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Vessel {
    fn clear(&mut self) {
        self.clear_width();
        self.clear_length();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Vessel {
    fn eq(&self, other: &Vessel) -> bool {
        self.width == other.width &&
        self.length == other.length &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Vessel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FuelTank {
    // message fields
    radius: ::std::option::Option<f64>,
    length: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl FuelTank {
    pub fn new() -> FuelTank {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FuelTank {
        static mut instance: ::protobuf::lazy::Lazy<FuelTank> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FuelTank,
        };
        unsafe {
            instance.get(|| {
                FuelTank {
                    radius: ::std::option::Option::None,
                    length: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required double radius = 1;

    pub fn clear_radius(&mut self) {
        self.radius = ::std::option::Option::None;
    }

    pub fn has_radius(&self) -> bool {
        self.radius.is_some()
    }

    // Param is passed by value, moved
    pub fn set_radius(&mut self, v: f64) {
        self.radius = ::std::option::Option::Some(v);
    }

    pub fn get_radius<'a>(&self) -> f64 {
        self.radius.unwrap_or(0.)
    }

    // required double length = 2;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: f64) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length<'a>(&self) -> f64 {
        self.length.unwrap_or(0.)
    }
}

impl ::protobuf::Message for FuelTank {
    fn is_initialized(&self) -> bool {
        if self.radius.is_none() {
            return false;
        };
        if self.length.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.radius = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.length = ::std::option::Option::Some(tmp);
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
        if self.radius.is_some() {
            my_size += 9;
        };
        if self.length.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.radius {
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.length {
            try!(os.write_double(2, v));
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
        ::std::any::TypeId::of::<FuelTank>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FuelTank {
    fn new() -> FuelTank {
        FuelTank::new()
    }

    fn descriptor_static(_: ::std::option::Option<FuelTank>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "radius",
                    FuelTank::has_radius,
                    FuelTank::get_radius,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "length",
                    FuelTank::has_length,
                    FuelTank::get_length,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FuelTank>(
                    "FuelTank",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FuelTank {
    fn clear(&mut self) {
        self.clear_radius();
        self.clear_length();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FuelTank {
    fn eq(&self, other: &FuelTank) -> bool {
        self.radius == other.radius &&
        self.length == other.length &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FuelTank {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Engine {
    // message fields
    radius: ::std::option::Option<f64>,
    length: ::std::option::Option<f64>,
    group: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Engine {
    pub fn new() -> Engine {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Engine {
        static mut instance: ::protobuf::lazy::Lazy<Engine> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Engine,
        };
        unsafe {
            instance.get(|| {
                Engine {
                    radius: ::std::option::Option::None,
                    length: ::std::option::Option::None,
                    group: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required double radius = 1;

    pub fn clear_radius(&mut self) {
        self.radius = ::std::option::Option::None;
    }

    pub fn has_radius(&self) -> bool {
        self.radius.is_some()
    }

    // Param is passed by value, moved
    pub fn set_radius(&mut self, v: f64) {
        self.radius = ::std::option::Option::Some(v);
    }

    pub fn get_radius<'a>(&self) -> f64 {
        self.radius.unwrap_or(0.)
    }

    // required double length = 2;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: f64) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length<'a>(&self) -> f64 {
        self.length.unwrap_or(0.)
    }

    // required int32 group = 3;

    pub fn clear_group(&mut self) {
        self.group = ::std::option::Option::None;
    }

    pub fn has_group(&self) -> bool {
        self.group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group(&mut self, v: i32) {
        self.group = ::std::option::Option::Some(v);
    }

    pub fn get_group<'a>(&self) -> i32 {
        self.group.unwrap_or(0)
    }
}

impl ::protobuf::Message for Engine {
    fn is_initialized(&self) -> bool {
        if self.radius.is_none() {
            return false;
        };
        if self.length.is_none() {
            return false;
        };
        if self.group.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.radius = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.length = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.group = ::std::option::Option::Some(tmp);
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
        if self.radius.is_some() {
            my_size += 9;
        };
        if self.length.is_some() {
            my_size += 9;
        };
        for value in self.group.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.radius {
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.length {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.group {
            try!(os.write_int32(3, v));
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
        ::std::any::TypeId::of::<Engine>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Engine {
    fn new() -> Engine {
        Engine::new()
    }

    fn descriptor_static(_: ::std::option::Option<Engine>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "radius",
                    Engine::has_radius,
                    Engine::get_radius,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "length",
                    Engine::has_length,
                    Engine::get_length,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "group",
                    Engine::has_group,
                    Engine::get_group,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Engine>(
                    "Engine",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Engine {
    fn clear(&mut self) {
        self.clear_radius();
        self.clear_length();
        self.clear_group();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Engine {
    fn eq(&self, other: &Engine) -> bool {
        self.radius == other.radius &&
        self.length == other.length &&
        self.group == other.group &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Engine {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Part {
    // message fields
    // message oneof groups
    part: ::std::option::Option<Part_oneof_part>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum Part_oneof_part {
    vessel(Vessel),
    fuelTank(FuelTank),
    engine(Engine),
}

impl Part {
    pub fn new() -> Part {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Part {
        static mut instance: ::protobuf::lazy::Lazy<Part> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Part,
        };
        unsafe {
            instance.get(|| {
                Part {
                    part: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .ship.Vessel vessel = 1;

    pub fn clear_vessel(&mut self) {
        self.part = ::std::option::Option::None;
    }

    pub fn has_vessel(&self) -> bool {
        match self.part {
            ::std::option::Option::Some(Part_oneof_part::vessel(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_vessel(&mut self, v: Vessel) {
        self.part = ::std::option::Option::Some(Part_oneof_part::vessel(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_vessel<'a>(&'a mut self) -> &'a mut Vessel {
        if let ::std::option::Option::Some(Part_oneof_part::vessel(_)) = self.part {
        } else {
            self.part = ::std::option::Option::Some(Part_oneof_part::vessel(Vessel::new()));
        }
        match self.part {
            ::std::option::Option::Some(Part_oneof_part::vessel(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_vessel(&mut self) -> Vessel {
        if self.has_vessel() {
            match self.part.take() {
                ::std::option::Option::Some(Part_oneof_part::vessel(v)) => v,
                _ => panic!(),
            }
        } else {
            Vessel::new()
        }
    }

    pub fn get_vessel<'a>(&'a self) -> &'a Vessel {
        match self.part {
            ::std::option::Option::Some(Part_oneof_part::vessel(ref v)) => v,
            _ => Vessel::default_instance(),
        }
    }

    // optional .ship.FuelTank fuelTank = 2;

    pub fn clear_fuelTank(&mut self) {
        self.part = ::std::option::Option::None;
    }

    pub fn has_fuelTank(&self) -> bool {
        match self.part {
            ::std::option::Option::Some(Part_oneof_part::fuelTank(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_fuelTank(&mut self, v: FuelTank) {
        self.part = ::std::option::Option::Some(Part_oneof_part::fuelTank(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fuelTank<'a>(&'a mut self) -> &'a mut FuelTank {
        if let ::std::option::Option::Some(Part_oneof_part::fuelTank(_)) = self.part {
        } else {
            self.part = ::std::option::Option::Some(Part_oneof_part::fuelTank(FuelTank::new()));
        }
        match self.part {
            ::std::option::Option::Some(Part_oneof_part::fuelTank(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_fuelTank(&mut self) -> FuelTank {
        if self.has_fuelTank() {
            match self.part.take() {
                ::std::option::Option::Some(Part_oneof_part::fuelTank(v)) => v,
                _ => panic!(),
            }
        } else {
            FuelTank::new()
        }
    }

    pub fn get_fuelTank<'a>(&'a self) -> &'a FuelTank {
        match self.part {
            ::std::option::Option::Some(Part_oneof_part::fuelTank(ref v)) => v,
            _ => FuelTank::default_instance(),
        }
    }

    // optional .ship.Engine engine = 3;

    pub fn clear_engine(&mut self) {
        self.part = ::std::option::Option::None;
    }

    pub fn has_engine(&self) -> bool {
        match self.part {
            ::std::option::Option::Some(Part_oneof_part::engine(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_engine(&mut self, v: Engine) {
        self.part = ::std::option::Option::Some(Part_oneof_part::engine(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_engine<'a>(&'a mut self) -> &'a mut Engine {
        if let ::std::option::Option::Some(Part_oneof_part::engine(_)) = self.part {
        } else {
            self.part = ::std::option::Option::Some(Part_oneof_part::engine(Engine::new()));
        }
        match self.part {
            ::std::option::Option::Some(Part_oneof_part::engine(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_engine(&mut self) -> Engine {
        if self.has_engine() {
            match self.part.take() {
                ::std::option::Option::Some(Part_oneof_part::engine(v)) => v,
                _ => panic!(),
            }
        } else {
            Engine::new()
        }
    }

    pub fn get_engine<'a>(&'a self) -> &'a Engine {
        match self.part {
            ::std::option::Option::Some(Part_oneof_part::engine(ref v)) => v,
            _ => Engine::default_instance(),
        }
    }
}

impl ::protobuf::Message for Part {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.part = ::std::option::Option::Some(Part_oneof_part::vessel(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.part = ::std::option::Option::Some(Part_oneof_part::fuelTank(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.part = ::std::option::Option::Some(Part_oneof_part::engine(try!(is.read_message())));
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
        if let ::std::option::Option::Some(ref v) = self.part {
            match v {
                &Part_oneof_part::vessel(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Part_oneof_part::fuelTank(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Part_oneof_part::engine(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.part {
            match v {
                &Part_oneof_part::vessel(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Part_oneof_part::fuelTank(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Part_oneof_part::engine(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<Part>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Part {
    fn new() -> Part {
        Part::new()
    }

    fn descriptor_static(_: ::std::option::Option<Part>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "vessel",
                    Part::has_vessel,
                    Part::get_vessel,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "fuelTank",
                    Part::has_fuelTank,
                    Part::get_fuelTank,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "engine",
                    Part::has_engine,
                    Part::get_engine,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Part>(
                    "Part",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Part {
    fn clear(&mut self) {
        self.clear_vessel();
        self.clear_fuelTank();
        self.clear_engine();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Part {
    fn eq(&self, other: &Part) -> bool {
        self.part == other.part &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Part {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Beam {
    // message fields
    length: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Beam {
    pub fn new() -> Beam {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Beam {
        static mut instance: ::protobuf::lazy::Lazy<Beam> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Beam,
        };
        unsafe {
            instance.get(|| {
                Beam {
                    length: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required double length = 1;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: f64) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length<'a>(&self) -> f64 {
        self.length.unwrap_or(0.)
    }
}

impl ::protobuf::Message for Beam {
    fn is_initialized(&self) -> bool {
        if self.length.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.length = ::std::option::Option::Some(tmp);
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
        if self.length.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.length {
            try!(os.write_double(1, v));
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
        ::std::any::TypeId::of::<Beam>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Beam {
    fn new() -> Beam {
        Beam::new()
    }

    fn descriptor_static(_: ::std::option::Option<Beam>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "length",
                    Beam::has_length,
                    Beam::get_length,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Beam>(
                    "Beam",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Beam {
    fn clear(&mut self) {
        self.clear_length();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Beam {
    fn eq(&self, other: &Beam) -> bool {
        self.length == other.length &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Beam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Attachment {
    // message fields
    // message oneof groups
    attachment: ::std::option::Option<Attachment_oneof_attachment>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum Attachment_oneof_attachment {
    beam(Beam),
    part(Part),
}

impl Attachment {
    pub fn new() -> Attachment {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Attachment {
        static mut instance: ::protobuf::lazy::Lazy<Attachment> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Attachment,
        };
        unsafe {
            instance.get(|| {
                Attachment {
                    attachment: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .ship.Beam beam = 1;

    pub fn clear_beam(&mut self) {
        self.attachment = ::std::option::Option::None;
    }

    pub fn has_beam(&self) -> bool {
        match self.attachment {
            ::std::option::Option::Some(Attachment_oneof_attachment::beam(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_beam(&mut self, v: Beam) {
        self.attachment = ::std::option::Option::Some(Attachment_oneof_attachment::beam(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_beam<'a>(&'a mut self) -> &'a mut Beam {
        if let ::std::option::Option::Some(Attachment_oneof_attachment::beam(_)) = self.attachment {
        } else {
            self.attachment = ::std::option::Option::Some(Attachment_oneof_attachment::beam(Beam::new()));
        }
        match self.attachment {
            ::std::option::Option::Some(Attachment_oneof_attachment::beam(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_beam(&mut self) -> Beam {
        if self.has_beam() {
            match self.attachment.take() {
                ::std::option::Option::Some(Attachment_oneof_attachment::beam(v)) => v,
                _ => panic!(),
            }
        } else {
            Beam::new()
        }
    }

    pub fn get_beam<'a>(&'a self) -> &'a Beam {
        match self.attachment {
            ::std::option::Option::Some(Attachment_oneof_attachment::beam(ref v)) => v,
            _ => Beam::default_instance(),
        }
    }

    // optional .ship.Part part = 2;

    pub fn clear_part(&mut self) {
        self.attachment = ::std::option::Option::None;
    }

    pub fn has_part(&self) -> bool {
        match self.attachment {
            ::std::option::Option::Some(Attachment_oneof_attachment::part(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_part(&mut self, v: Part) {
        self.attachment = ::std::option::Option::Some(Attachment_oneof_attachment::part(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_part<'a>(&'a mut self) -> &'a mut Part {
        if let ::std::option::Option::Some(Attachment_oneof_attachment::part(_)) = self.attachment {
        } else {
            self.attachment = ::std::option::Option::Some(Attachment_oneof_attachment::part(Part::new()));
        }
        match self.attachment {
            ::std::option::Option::Some(Attachment_oneof_attachment::part(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_part(&mut self) -> Part {
        if self.has_part() {
            match self.attachment.take() {
                ::std::option::Option::Some(Attachment_oneof_attachment::part(v)) => v,
                _ => panic!(),
            }
        } else {
            Part::new()
        }
    }

    pub fn get_part<'a>(&'a self) -> &'a Part {
        match self.attachment {
            ::std::option::Option::Some(Attachment_oneof_attachment::part(ref v)) => v,
            _ => Part::default_instance(),
        }
    }
}

impl ::protobuf::Message for Attachment {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.attachment = ::std::option::Option::Some(Attachment_oneof_attachment::beam(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.attachment = ::std::option::Option::Some(Attachment_oneof_attachment::part(try!(is.read_message())));
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
        if let ::std::option::Option::Some(ref v) = self.attachment {
            match v {
                &Attachment_oneof_attachment::beam(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Attachment_oneof_attachment::part(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.attachment {
            match v {
                &Attachment_oneof_attachment::beam(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Attachment_oneof_attachment::part(ref v) => {
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
        ::std::any::TypeId::of::<Attachment>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Attachment {
    fn new() -> Attachment {
        Attachment::new()
    }

    fn descriptor_static(_: ::std::option::Option<Attachment>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "beam",
                    Attachment::has_beam,
                    Attachment::get_beam,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "part",
                    Attachment::has_part,
                    Attachment::get_part,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Attachment>(
                    "Attachment",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Attachment {
    fn clear(&mut self) {
        self.clear_beam();
        self.clear_part();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Attachment {
    fn eq(&self, other: &Attachment) -> bool {
        self.attachment == other.attachment &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Attachment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Attach {
    // message fields
    identity: ::std::option::Option<i32>,
    location: ::std::option::Option<f64>,
    rotation: ::std::option::Option<f64>,
    attachment: ::protobuf::SingularPtrField<Attachment>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Attach {
    pub fn new() -> Attach {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Attach {
        static mut instance: ::protobuf::lazy::Lazy<Attach> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Attach,
        };
        unsafe {
            instance.get(|| {
                Attach {
                    identity: ::std::option::Option::None,
                    location: ::std::option::Option::None,
                    rotation: ::std::option::Option::None,
                    attachment: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 identity = 1;

    pub fn clear_identity(&mut self) {
        self.identity = ::std::option::Option::None;
    }

    pub fn has_identity(&self) -> bool {
        self.identity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_identity(&mut self, v: i32) {
        self.identity = ::std::option::Option::Some(v);
    }

    pub fn get_identity<'a>(&self) -> i32 {
        self.identity.unwrap_or(0)
    }

    // required double location = 2;

    pub fn clear_location(&mut self) {
        self.location = ::std::option::Option::None;
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: f64) {
        self.location = ::std::option::Option::Some(v);
    }

    pub fn get_location<'a>(&self) -> f64 {
        self.location.unwrap_or(0.)
    }

    // required double rotation = 3;

    pub fn clear_rotation(&mut self) {
        self.rotation = ::std::option::Option::None;
    }

    pub fn has_rotation(&self) -> bool {
        self.rotation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rotation(&mut self, v: f64) {
        self.rotation = ::std::option::Option::Some(v);
    }

    pub fn get_rotation<'a>(&self) -> f64 {
        self.rotation.unwrap_or(0.)
    }

    // required .ship.Attachment attachment = 4;

    pub fn clear_attachment(&mut self) {
        self.attachment.clear();
    }

    pub fn has_attachment(&self) -> bool {
        self.attachment.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attachment(&mut self, v: Attachment) {
        self.attachment = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_attachment<'a>(&'a mut self) -> &'a mut Attachment {
        if self.attachment.is_none() {
            self.attachment.set_default();
        };
        self.attachment.as_mut().unwrap()
    }

    // Take field
    pub fn take_attachment(&mut self) -> Attachment {
        self.attachment.take().unwrap_or_else(|| Attachment::new())
    }

    pub fn get_attachment<'a>(&'a self) -> &'a Attachment {
        self.attachment.as_ref().unwrap_or_else(|| Attachment::default_instance())
    }
}

impl ::protobuf::Message for Attach {
    fn is_initialized(&self) -> bool {
        if self.identity.is_none() {
            return false;
        };
        if self.location.is_none() {
            return false;
        };
        if self.rotation.is_none() {
            return false;
        };
        if self.attachment.is_none() {
            return false;
        };
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
                    let tmp = try!(is.read_int32());
                    self.identity = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.location = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.rotation = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.attachment.set_default();
                    try!(is.merge_message(tmp))
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
        for value in self.identity.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.location.is_some() {
            my_size += 9;
        };
        if self.rotation.is_some() {
            my_size += 9;
        };
        for value in self.attachment.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.identity {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.location {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.rotation {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.attachment.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<Attach>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Attach {
    fn new() -> Attach {
        Attach::new()
    }

    fn descriptor_static(_: ::std::option::Option<Attach>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "identity",
                    Attach::has_identity,
                    Attach::get_identity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "location",
                    Attach::has_location,
                    Attach::get_location,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "rotation",
                    Attach::has_rotation,
                    Attach::get_rotation,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "attachment",
                    Attach::has_attachment,
                    Attach::get_attachment,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Attach>(
                    "Attach",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Attach {
    fn clear(&mut self) {
        self.clear_identity();
        self.clear_location();
        self.clear_rotation();
        self.clear_attachment();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Attach {
    fn eq(&self, other: &Attach) -> bool {
        self.identity == other.identity &&
        self.location == other.location &&
        self.rotation == other.rotation &&
        self.attachment == other.attachment &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Attach {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Structure {
    // message fields
    attachments: ::protobuf::RepeatedField<Attach>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Structure {
    pub fn new() -> Structure {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Structure {
        static mut instance: ::protobuf::lazy::Lazy<Structure> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Structure,
        };
        unsafe {
            instance.get(|| {
                Structure {
                    attachments: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .ship.Attach attachments = 1;

    pub fn clear_attachments(&mut self) {
        self.attachments.clear();
    }

    // Param is passed by value, moved
    pub fn set_attachments(&mut self, v: ::protobuf::RepeatedField<Attach>) {
        self.attachments = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attachments<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Attach> {
        &mut self.attachments
    }

    // Take field
    pub fn take_attachments(&mut self) -> ::protobuf::RepeatedField<Attach> {
        ::std::mem::replace(&mut self.attachments, ::protobuf::RepeatedField::new())
    }

    pub fn get_attachments<'a>(&'a self) -> &'a [Attach] {
        &self.attachments
    }
}

impl ::protobuf::Message for Structure {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.attachments));
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
        for value in self.attachments.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.attachments.iter() {
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
        ::std::any::TypeId::of::<Structure>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Structure {
    fn new() -> Structure {
        Structure::new()
    }

    fn descriptor_static(_: ::std::option::Option<Structure>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "attachments",
                    Structure::get_attachments,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Structure>(
                    "Structure",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Structure {
    fn clear(&mut self) {
        self.clear_attachments();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Structure {
    fn eq(&self, other: &Structure) -> bool {
        self.attachments == other.attachments &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Structure {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0e, 0x73, 0x72, 0x63, 0x2f, 0x73, 0x68, 0x69, 0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x04, 0x73, 0x68, 0x69, 0x70, 0x22, 0x27, 0x0a, 0x06, 0x56, 0x65, 0x73, 0x73, 0x65, 0x6c,
    0x12, 0x0d, 0x0a, 0x05, 0x77, 0x69, 0x64, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x01, 0x12,
    0x0e, 0x0a, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x18, 0x02, 0x20, 0x02, 0x28, 0x01, 0x22,
    0x2a, 0x0a, 0x08, 0x46, 0x75, 0x65, 0x6c, 0x54, 0x61, 0x6e, 0x6b, 0x12, 0x0e, 0x0a, 0x06, 0x72,
    0x61, 0x64, 0x69, 0x75, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x01, 0x12, 0x0e, 0x0a, 0x06, 0x6c,
    0x65, 0x6e, 0x67, 0x74, 0x68, 0x18, 0x02, 0x20, 0x02, 0x28, 0x01, 0x22, 0x37, 0x0a, 0x06, 0x45,
    0x6e, 0x67, 0x69, 0x6e, 0x65, 0x12, 0x0e, 0x0a, 0x06, 0x72, 0x61, 0x64, 0x69, 0x75, 0x73, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x01, 0x12, 0x0e, 0x0a, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x01, 0x12, 0x0d, 0x0a, 0x05, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x18, 0x03,
    0x20, 0x02, 0x28, 0x05, 0x22, 0x72, 0x0a, 0x04, 0x50, 0x61, 0x72, 0x74, 0x12, 0x1e, 0x0a, 0x06,
    0x76, 0x65, 0x73, 0x73, 0x65, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x73,
    0x68, 0x69, 0x70, 0x2e, 0x56, 0x65, 0x73, 0x73, 0x65, 0x6c, 0x48, 0x00, 0x12, 0x22, 0x0a, 0x08,
    0x66, 0x75, 0x65, 0x6c, 0x54, 0x61, 0x6e, 0x6b, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e,
    0x2e, 0x73, 0x68, 0x69, 0x70, 0x2e, 0x46, 0x75, 0x65, 0x6c, 0x54, 0x61, 0x6e, 0x6b, 0x48, 0x00,
    0x12, 0x1e, 0x0a, 0x06, 0x65, 0x6e, 0x67, 0x69, 0x6e, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x0c, 0x2e, 0x73, 0x68, 0x69, 0x70, 0x2e, 0x45, 0x6e, 0x67, 0x69, 0x6e, 0x65, 0x48, 0x00,
    0x42, 0x06, 0x0a, 0x04, 0x70, 0x61, 0x72, 0x74, 0x22, 0x16, 0x0a, 0x04, 0x42, 0x65, 0x61, 0x6d,
    0x12, 0x0e, 0x0a, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x01,
    0x22, 0x52, 0x0a, 0x0a, 0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x12, 0x1a,
    0x0a, 0x04, 0x62, 0x65, 0x61, 0x6d, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x73,
    0x68, 0x69, 0x70, 0x2e, 0x42, 0x65, 0x61, 0x6d, 0x48, 0x00, 0x12, 0x1a, 0x0a, 0x04, 0x70, 0x61,
    0x72, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x73, 0x68, 0x69, 0x70, 0x2e,
    0x50, 0x61, 0x72, 0x74, 0x48, 0x00, 0x42, 0x0c, 0x0a, 0x0a, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68,
    0x6d, 0x65, 0x6e, 0x74, 0x22, 0x64, 0x0a, 0x06, 0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x12, 0x10,
    0x0a, 0x08, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x18, 0x01, 0x20, 0x02, 0x28, 0x05,
    0x12, 0x10, 0x0a, 0x08, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x01, 0x12, 0x10, 0x0a, 0x08, 0x72, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03,
    0x20, 0x02, 0x28, 0x01, 0x12, 0x24, 0x0a, 0x0a, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65,
    0x6e, 0x74, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x73, 0x68, 0x69, 0x70, 0x2e,
    0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x22, 0x2e, 0x0a, 0x09, 0x53, 0x74,
    0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x12, 0x21, 0x0a, 0x0b, 0x61, 0x74, 0x74, 0x61, 0x63,
    0x68, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x73,
    0x68, 0x69, 0x70, 0x2e, 0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x4a, 0x9e, 0x0b, 0x0a, 0x06, 0x12,
    0x04, 0x01, 0x00, 0x2f, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x0c, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x03, 0x00, 0x06, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x04, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x04,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x04, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x12, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x04, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x05, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x05, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x05, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x05, 0x1b,
    0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x08, 0x00, 0x0b, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x08, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x09, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x12, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x1b, 0x1c, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x0a, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x0a, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0d, 0x00, 0x11, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x00, 0x12, 0x03, 0x0e, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e,
    0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x1b, 0x1c,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x02, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x0f, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x10,
    0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x10, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x10, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03,
    0x12, 0x04, 0x13, 0x00, 0x19, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x13,
    0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x08, 0x00, 0x12, 0x04, 0x14, 0x02, 0x18, 0x03,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x08, 0x00, 0x01, 0x12, 0x03, 0x14, 0x08, 0x0c, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x15, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x15, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x15, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x15, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x16,
    0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x16, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x16, 0x0d, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x16, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x17, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x02, 0x06, 0x12, 0x03, 0x17, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x17, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x17, 0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x1b, 0x00, 0x1d, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x1c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x1c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c,
    0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1c, 0x1b, 0x1c,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x1f, 0x00, 0x24, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x05, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x08, 0x00,
    0x12, 0x04, 0x20, 0x02, 0x23, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x08, 0x00, 0x01, 0x12,
    0x03, 0x20, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x21, 0x04,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x21, 0x04, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x09, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x21, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x01, 0x12, 0x03, 0x22, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x22, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x22, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x22,
    0x10, 0x11, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x26, 0x00, 0x2b, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x26, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x00, 0x12, 0x03, 0x27, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x27, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x27, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x27, 0x11,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x27, 0x1c, 0x1d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x28, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x28, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x28, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x28, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x28, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x29, 0x02,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x29, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x05, 0x12, 0x03, 0x29, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x29, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x29, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02,
    0x03, 0x12, 0x03, 0x2a, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x2a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x06, 0x12, 0x03, 0x2a,
    0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2a, 0x16, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2a, 0x23, 0x24, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x2d, 0x00, 0x2f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07,
    0x01, 0x12, 0x03, 0x2d, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03,
    0x2e, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2e, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2e, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2e, 0x12, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2e, 0x20, 0x21,
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
