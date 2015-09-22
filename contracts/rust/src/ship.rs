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
pub struct Root {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Root {
    pub fn new() -> Root {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Root {
        static mut instance: ::protobuf::lazy::Lazy<Root> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Root,
        };
        unsafe {
            instance.get(|| {
                Root {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for Root {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::any::TypeId::of::<Root>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Root {
    fn new() -> Root {
        Root::new()
    }

    fn descriptor_static(_: ::std::option::Option<Root>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Root>(
                    "Root",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Root {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Root {
    fn eq(&self, other: &Root) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Root {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Attach {
    // message fields
    location: ::std::option::Option<f64>,
    rotation: ::std::option::Option<f64>,
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
                    location: ::std::option::Option::None,
                    rotation: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required double location = 1;

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

    // required double rotation = 2;

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
}

impl ::protobuf::Message for Attach {
    fn is_initialized(&self) -> bool {
        if self.location.is_none() {
            return false;
        };
        if self.rotation.is_none() {
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
                    self.location = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.rotation = ::std::option::Option::Some(tmp);
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
        if self.location.is_some() {
            my_size += 9;
        };
        if self.rotation.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.location {
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.rotation {
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
        self.clear_location();
        self.clear_rotation();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Attach {
    fn eq(&self, other: &Attach) -> bool {
        self.location == other.location &&
        self.rotation == other.rotation &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Attach {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StructureNode {
    // message fields
    // message oneof groups
    node: ::std::option::Option<StructureNode_oneof_node>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum StructureNode_oneof_node {
    beam(Beam),
    part(Part),
}

impl StructureNode {
    pub fn new() -> StructureNode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StructureNode {
        static mut instance: ::protobuf::lazy::Lazy<StructureNode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StructureNode,
        };
        unsafe {
            instance.get(|| {
                StructureNode {
                    node: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .ship.Beam beam = 1;

    pub fn clear_beam(&mut self) {
        self.node = ::std::option::Option::None;
    }

    pub fn has_beam(&self) -> bool {
        match self.node {
            ::std::option::Option::Some(StructureNode_oneof_node::beam(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_beam(&mut self, v: Beam) {
        self.node = ::std::option::Option::Some(StructureNode_oneof_node::beam(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_beam<'a>(&'a mut self) -> &'a mut Beam {
        if let ::std::option::Option::Some(StructureNode_oneof_node::beam(_)) = self.node {
        } else {
            self.node = ::std::option::Option::Some(StructureNode_oneof_node::beam(Beam::new()));
        }
        match self.node {
            ::std::option::Option::Some(StructureNode_oneof_node::beam(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_beam(&mut self) -> Beam {
        if self.has_beam() {
            match self.node.take() {
                ::std::option::Option::Some(StructureNode_oneof_node::beam(v)) => v,
                _ => panic!(),
            }
        } else {
            Beam::new()
        }
    }

    pub fn get_beam<'a>(&'a self) -> &'a Beam {
        match self.node {
            ::std::option::Option::Some(StructureNode_oneof_node::beam(ref v)) => v,
            _ => Beam::default_instance(),
        }
    }

    // optional .ship.Part part = 2;

    pub fn clear_part(&mut self) {
        self.node = ::std::option::Option::None;
    }

    pub fn has_part(&self) -> bool {
        match self.node {
            ::std::option::Option::Some(StructureNode_oneof_node::part(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_part(&mut self, v: Part) {
        self.node = ::std::option::Option::Some(StructureNode_oneof_node::part(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_part<'a>(&'a mut self) -> &'a mut Part {
        if let ::std::option::Option::Some(StructureNode_oneof_node::part(_)) = self.node {
        } else {
            self.node = ::std::option::Option::Some(StructureNode_oneof_node::part(Part::new()));
        }
        match self.node {
            ::std::option::Option::Some(StructureNode_oneof_node::part(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_part(&mut self) -> Part {
        if self.has_part() {
            match self.node.take() {
                ::std::option::Option::Some(StructureNode_oneof_node::part(v)) => v,
                _ => panic!(),
            }
        } else {
            Part::new()
        }
    }

    pub fn get_part<'a>(&'a self) -> &'a Part {
        match self.node {
            ::std::option::Option::Some(StructureNode_oneof_node::part(ref v)) => v,
            _ => Part::default_instance(),
        }
    }
}

impl ::protobuf::Message for StructureNode {
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
                    self.node = ::std::option::Option::Some(StructureNode_oneof_node::beam(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.node = ::std::option::Option::Some(StructureNode_oneof_node::part(try!(is.read_message())));
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
        if let ::std::option::Option::Some(ref v) = self.node {
            match v {
                &StructureNode_oneof_node::beam(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &StructureNode_oneof_node::part(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.node {
            match v {
                &StructureNode_oneof_node::beam(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &StructureNode_oneof_node::part(ref v) => {
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
        ::std::any::TypeId::of::<StructureNode>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StructureNode {
    fn new() -> StructureNode {
        StructureNode::new()
    }

    fn descriptor_static(_: ::std::option::Option<StructureNode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "beam",
                    StructureNode::has_beam,
                    StructureNode::get_beam,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "part",
                    StructureNode::has_part,
                    StructureNode::get_part,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StructureNode>(
                    "StructureNode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StructureNode {
    fn clear(&mut self) {
        self.clear_beam();
        self.clear_part();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StructureNode {
    fn eq(&self, other: &StructureNode) -> bool {
        self.node == other.node &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StructureNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StructureLink {
    // message fields
    // message oneof groups
    link: ::std::option::Option<StructureLink_oneof_link>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum StructureLink_oneof_link {
    root(Root),
    attach(Attach),
}

impl StructureLink {
    pub fn new() -> StructureLink {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StructureLink {
        static mut instance: ::protobuf::lazy::Lazy<StructureLink> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StructureLink,
        };
        unsafe {
            instance.get(|| {
                StructureLink {
                    link: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .ship.Root root = 1;

    pub fn clear_root(&mut self) {
        self.link = ::std::option::Option::None;
    }

    pub fn has_root(&self) -> bool {
        match self.link {
            ::std::option::Option::Some(StructureLink_oneof_link::root(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_root(&mut self, v: Root) {
        self.link = ::std::option::Option::Some(StructureLink_oneof_link::root(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_root<'a>(&'a mut self) -> &'a mut Root {
        if let ::std::option::Option::Some(StructureLink_oneof_link::root(_)) = self.link {
        } else {
            self.link = ::std::option::Option::Some(StructureLink_oneof_link::root(Root::new()));
        }
        match self.link {
            ::std::option::Option::Some(StructureLink_oneof_link::root(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_root(&mut self) -> Root {
        if self.has_root() {
            match self.link.take() {
                ::std::option::Option::Some(StructureLink_oneof_link::root(v)) => v,
                _ => panic!(),
            }
        } else {
            Root::new()
        }
    }

    pub fn get_root<'a>(&'a self) -> &'a Root {
        match self.link {
            ::std::option::Option::Some(StructureLink_oneof_link::root(ref v)) => v,
            _ => Root::default_instance(),
        }
    }

    // optional .ship.Attach attach = 2;

    pub fn clear_attach(&mut self) {
        self.link = ::std::option::Option::None;
    }

    pub fn has_attach(&self) -> bool {
        match self.link {
            ::std::option::Option::Some(StructureLink_oneof_link::attach(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_attach(&mut self, v: Attach) {
        self.link = ::std::option::Option::Some(StructureLink_oneof_link::attach(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_attach<'a>(&'a mut self) -> &'a mut Attach {
        if let ::std::option::Option::Some(StructureLink_oneof_link::attach(_)) = self.link {
        } else {
            self.link = ::std::option::Option::Some(StructureLink_oneof_link::attach(Attach::new()));
        }
        match self.link {
            ::std::option::Option::Some(StructureLink_oneof_link::attach(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_attach(&mut self) -> Attach {
        if self.has_attach() {
            match self.link.take() {
                ::std::option::Option::Some(StructureLink_oneof_link::attach(v)) => v,
                _ => panic!(),
            }
        } else {
            Attach::new()
        }
    }

    pub fn get_attach<'a>(&'a self) -> &'a Attach {
        match self.link {
            ::std::option::Option::Some(StructureLink_oneof_link::attach(ref v)) => v,
            _ => Attach::default_instance(),
        }
    }
}

impl ::protobuf::Message for StructureLink {
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
                    self.link = ::std::option::Option::Some(StructureLink_oneof_link::root(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.link = ::std::option::Option::Some(StructureLink_oneof_link::attach(try!(is.read_message())));
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
        if let ::std::option::Option::Some(ref v) = self.link {
            match v {
                &StructureLink_oneof_link::root(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &StructureLink_oneof_link::attach(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.link {
            match v {
                &StructureLink_oneof_link::root(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &StructureLink_oneof_link::attach(ref v) => {
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
        ::std::any::TypeId::of::<StructureLink>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StructureLink {
    fn new() -> StructureLink {
        StructureLink::new()
    }

    fn descriptor_static(_: ::std::option::Option<StructureLink>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "root",
                    StructureLink::has_root,
                    StructureLink::get_root,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "attach",
                    StructureLink::has_attach,
                    StructureLink::get_attach,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StructureLink>(
                    "StructureLink",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StructureLink {
    fn clear(&mut self) {
        self.clear_root();
        self.clear_attach();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StructureLink {
    fn eq(&self, other: &StructureLink) -> bool {
        self.link == other.link &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StructureLink {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StructureTree {
    // message fields
    node: ::protobuf::SingularPtrField<StructureNode>,
    link: ::protobuf::SingularPtrField<StructureLink>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StructureTree {
    pub fn new() -> StructureTree {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StructureTree {
        static mut instance: ::protobuf::lazy::Lazy<StructureTree> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StructureTree,
        };
        unsafe {
            instance.get(|| {
                StructureTree {
                    node: ::protobuf::SingularPtrField::none(),
                    link: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .ship.StructureNode node = 1;

    pub fn clear_node(&mut self) {
        self.node.clear();
    }

    pub fn has_node(&self) -> bool {
        self.node.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node(&mut self, v: StructureNode) {
        self.node = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node<'a>(&'a mut self) -> &'a mut StructureNode {
        if self.node.is_none() {
            self.node.set_default();
        };
        self.node.as_mut().unwrap()
    }

    // Take field
    pub fn take_node(&mut self) -> StructureNode {
        self.node.take().unwrap_or_else(|| StructureNode::new())
    }

    pub fn get_node<'a>(&'a self) -> &'a StructureNode {
        self.node.as_ref().unwrap_or_else(|| StructureNode::default_instance())
    }

    // required .ship.StructureLink link = 2;

    pub fn clear_link(&mut self) {
        self.link.clear();
    }

    pub fn has_link(&self) -> bool {
        self.link.is_some()
    }

    // Param is passed by value, moved
    pub fn set_link(&mut self, v: StructureLink) {
        self.link = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_link<'a>(&'a mut self) -> &'a mut StructureLink {
        if self.link.is_none() {
            self.link.set_default();
        };
        self.link.as_mut().unwrap()
    }

    // Take field
    pub fn take_link(&mut self) -> StructureLink {
        self.link.take().unwrap_or_else(|| StructureLink::new())
    }

    pub fn get_link<'a>(&'a self) -> &'a StructureLink {
        self.link.as_ref().unwrap_or_else(|| StructureLink::default_instance())
    }
}

impl ::protobuf::Message for StructureTree {
    fn is_initialized(&self) -> bool {
        if self.node.is_none() {
            return false;
        };
        if self.link.is_none() {
            return false;
        };
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
                    let tmp = self.node.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.link.set_default();
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
        for value in self.node.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.link.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.node.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.link.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<StructureTree>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StructureTree {
    fn new() -> StructureTree {
        StructureTree::new()
    }

    fn descriptor_static(_: ::std::option::Option<StructureTree>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "node",
                    StructureTree::has_node,
                    StructureTree::get_node,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "link",
                    StructureTree::has_link,
                    StructureTree::get_link,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StructureTree>(
                    "StructureTree",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StructureTree {
    fn clear(&mut self) {
        self.clear_node();
        self.clear_link();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StructureTree {
    fn eq(&self, other: &StructureTree) -> bool {
        self.node == other.node &&
        self.link == other.link &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StructureTree {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct EndMarker {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl EndMarker {
    pub fn new() -> EndMarker {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EndMarker {
        static mut instance: ::protobuf::lazy::Lazy<EndMarker> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EndMarker,
        };
        unsafe {
            instance.get(|| {
                EndMarker {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for EndMarker {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::any::TypeId::of::<EndMarker>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EndMarker {
    fn new() -> EndMarker {
        EndMarker::new()
    }

    fn descriptor_static(_: ::std::option::Option<EndMarker>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<EndMarker>(
                    "EndMarker",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EndMarker {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EndMarker {
    fn eq(&self, other: &EndMarker) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EndMarker {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StructureData {
    // message fields
    // message oneof groups
    structure: ::std::option::Option<StructureData_oneof_structure>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum StructureData_oneof_structure {
    marker(EndMarker),
    tree(StructureTree),
}

impl StructureData {
    pub fn new() -> StructureData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StructureData {
        static mut instance: ::protobuf::lazy::Lazy<StructureData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StructureData,
        };
        unsafe {
            instance.get(|| {
                StructureData {
                    structure: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .ship.EndMarker marker = 1;

    pub fn clear_marker(&mut self) {
        self.structure = ::std::option::Option::None;
    }

    pub fn has_marker(&self) -> bool {
        match self.structure {
            ::std::option::Option::Some(StructureData_oneof_structure::marker(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_marker(&mut self, v: EndMarker) {
        self.structure = ::std::option::Option::Some(StructureData_oneof_structure::marker(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_marker<'a>(&'a mut self) -> &'a mut EndMarker {
        if let ::std::option::Option::Some(StructureData_oneof_structure::marker(_)) = self.structure {
        } else {
            self.structure = ::std::option::Option::Some(StructureData_oneof_structure::marker(EndMarker::new()));
        }
        match self.structure {
            ::std::option::Option::Some(StructureData_oneof_structure::marker(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_marker(&mut self) -> EndMarker {
        if self.has_marker() {
            match self.structure.take() {
                ::std::option::Option::Some(StructureData_oneof_structure::marker(v)) => v,
                _ => panic!(),
            }
        } else {
            EndMarker::new()
        }
    }

    pub fn get_marker<'a>(&'a self) -> &'a EndMarker {
        match self.structure {
            ::std::option::Option::Some(StructureData_oneof_structure::marker(ref v)) => v,
            _ => EndMarker::default_instance(),
        }
    }

    // optional .ship.StructureTree tree = 2;

    pub fn clear_tree(&mut self) {
        self.structure = ::std::option::Option::None;
    }

    pub fn has_tree(&self) -> bool {
        match self.structure {
            ::std::option::Option::Some(StructureData_oneof_structure::tree(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tree(&mut self, v: StructureTree) {
        self.structure = ::std::option::Option::Some(StructureData_oneof_structure::tree(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tree<'a>(&'a mut self) -> &'a mut StructureTree {
        if let ::std::option::Option::Some(StructureData_oneof_structure::tree(_)) = self.structure {
        } else {
            self.structure = ::std::option::Option::Some(StructureData_oneof_structure::tree(StructureTree::new()));
        }
        match self.structure {
            ::std::option::Option::Some(StructureData_oneof_structure::tree(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_tree(&mut self) -> StructureTree {
        if self.has_tree() {
            match self.structure.take() {
                ::std::option::Option::Some(StructureData_oneof_structure::tree(v)) => v,
                _ => panic!(),
            }
        } else {
            StructureTree::new()
        }
    }

    pub fn get_tree<'a>(&'a self) -> &'a StructureTree {
        match self.structure {
            ::std::option::Option::Some(StructureData_oneof_structure::tree(ref v)) => v,
            _ => StructureTree::default_instance(),
        }
    }
}

impl ::protobuf::Message for StructureData {
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
                    self.structure = ::std::option::Option::Some(StructureData_oneof_structure::marker(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.structure = ::std::option::Option::Some(StructureData_oneof_structure::tree(try!(is.read_message())));
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
        if let ::std::option::Option::Some(ref v) = self.structure {
            match v {
                &StructureData_oneof_structure::marker(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &StructureData_oneof_structure::tree(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.structure {
            match v {
                &StructureData_oneof_structure::marker(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &StructureData_oneof_structure::tree(ref v) => {
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
        ::std::any::TypeId::of::<StructureData>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StructureData {
    fn new() -> StructureData {
        StructureData::new()
    }

    fn descriptor_static(_: ::std::option::Option<StructureData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "marker",
                    StructureData::has_marker,
                    StructureData::get_marker,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "tree",
                    StructureData::has_tree,
                    StructureData::get_tree,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StructureData>(
                    "StructureData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StructureData {
    fn clear(&mut self) {
        self.clear_marker();
        self.clear_tree();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StructureData {
    fn eq(&self, other: &StructureData) -> bool {
        self.structure == other.structure &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StructureData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Structure {
    // message fields
    attachments: ::protobuf::RepeatedField<StructureData>,
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

    // repeated .ship.StructureData attachments = 1;

    pub fn clear_attachments(&mut self) {
        self.attachments.clear();
    }

    // Param is passed by value, moved
    pub fn set_attachments(&mut self, v: ::protobuf::RepeatedField<StructureData>) {
        self.attachments = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attachments<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<StructureData> {
        &mut self.attachments
    }

    // Take field
    pub fn take_attachments(&mut self) -> ::protobuf::RepeatedField<StructureData> {
        ::std::mem::replace(&mut self.attachments, ::protobuf::RepeatedField::new())
    }

    pub fn get_attachments<'a>(&'a self) -> &'a [StructureData] {
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
    0x22, 0x06, 0x0a, 0x04, 0x52, 0x6f, 0x6f, 0x74, 0x22, 0x2c, 0x0a, 0x06, 0x41, 0x74, 0x74, 0x61,
    0x63, 0x68, 0x12, 0x10, 0x0a, 0x08, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x01, 0x12, 0x10, 0x0a, 0x08, 0x72, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x18, 0x02, 0x20, 0x02, 0x28, 0x01, 0x22, 0x4f, 0x0a, 0x0d, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74,
    0x75, 0x72, 0x65, 0x4e, 0x6f, 0x64, 0x65, 0x12, 0x1a, 0x0a, 0x04, 0x62, 0x65, 0x61, 0x6d, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x73, 0x68, 0x69, 0x70, 0x2e, 0x42, 0x65, 0x61,
    0x6d, 0x48, 0x00, 0x12, 0x1a, 0x0a, 0x04, 0x70, 0x61, 0x72, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0a, 0x2e, 0x73, 0x68, 0x69, 0x70, 0x2e, 0x50, 0x61, 0x72, 0x74, 0x48, 0x00, 0x42,
    0x06, 0x0a, 0x04, 0x6e, 0x6f, 0x64, 0x65, 0x22, 0x53, 0x0a, 0x0d, 0x53, 0x74, 0x72, 0x75, 0x63,
    0x74, 0x75, 0x72, 0x65, 0x4c, 0x69, 0x6e, 0x6b, 0x12, 0x1a, 0x0a, 0x04, 0x72, 0x6f, 0x6f, 0x74,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x73, 0x68, 0x69, 0x70, 0x2e, 0x52, 0x6f,
    0x6f, 0x74, 0x48, 0x00, 0x12, 0x1e, 0x0a, 0x06, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x73, 0x68, 0x69, 0x70, 0x2e, 0x41, 0x74, 0x74, 0x61,
    0x63, 0x68, 0x48, 0x00, 0x42, 0x06, 0x0a, 0x04, 0x6c, 0x69, 0x6e, 0x6b, 0x22, 0x55, 0x0a, 0x0d,
    0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x54, 0x72, 0x65, 0x65, 0x12, 0x21, 0x0a,
    0x04, 0x6e, 0x6f, 0x64, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x73, 0x68,
    0x69, 0x70, 0x2e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x4e, 0x6f, 0x64, 0x65,
    0x12, 0x21, 0x0a, 0x04, 0x6c, 0x69, 0x6e, 0x6b, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x13,
    0x2e, 0x73, 0x68, 0x69, 0x70, 0x2e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x4c,
    0x69, 0x6e, 0x6b, 0x22, 0x0b, 0x0a, 0x09, 0x45, 0x6e, 0x64, 0x4d, 0x61, 0x72, 0x6b, 0x65, 0x72,
    0x22, 0x64, 0x0a, 0x0d, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x44, 0x61, 0x74,
    0x61, 0x12, 0x21, 0x0a, 0x06, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0f, 0x2e, 0x73, 0x68, 0x69, 0x70, 0x2e, 0x45, 0x6e, 0x64, 0x4d, 0x61, 0x72, 0x6b,
    0x65, 0x72, 0x48, 0x00, 0x12, 0x23, 0x0a, 0x04, 0x74, 0x72, 0x65, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x13, 0x2e, 0x73, 0x68, 0x69, 0x70, 0x2e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74,
    0x75, 0x72, 0x65, 0x54, 0x72, 0x65, 0x65, 0x48, 0x00, 0x42, 0x0b, 0x0a, 0x09, 0x73, 0x74, 0x72,
    0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x22, 0x35, 0x0a, 0x09, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74,
    0x75, 0x72, 0x65, 0x12, 0x28, 0x0a, 0x0b, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e,
    0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x73, 0x68, 0x69, 0x70, 0x2e,
    0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x44, 0x61, 0x74, 0x61, 0x4a, 0xa8, 0x0e,
    0x0a, 0x06, 0x12, 0x04, 0x01, 0x00, 0x44, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01,
    0x08, 0x0c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x03, 0x00, 0x06, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x04, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x04, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x04, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x12,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x04, 0x1a, 0x1b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x05, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x05, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x05, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x05, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x08, 0x00, 0x0b, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x08, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x09, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x09, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x1b,
    0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x02, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x0a, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0d, 0x00,
    0x11, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x0e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x0e, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x0e, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x0e, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x02, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x0f, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02,
    0x12, 0x03, 0x10, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x10, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x10, 0x11, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x10, 0x19, 0x1a, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x03, 0x12, 0x04, 0x13, 0x00, 0x19, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01,
    0x12, 0x03, 0x13, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x08, 0x00, 0x12, 0x04, 0x14,
    0x02, 0x18, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x08, 0x00, 0x01, 0x12, 0x03, 0x14, 0x08,
    0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x15, 0x04, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x15, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x15, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01,
    0x12, 0x03, 0x16, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x16, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x16, 0x0d,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x16, 0x18, 0x19, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x17, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x02, 0x06, 0x12, 0x03, 0x17, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x17, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x17, 0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x1b, 0x00,
    0x1d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x0c, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x1c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x1c, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x1c, 0x1b, 0x1c, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x03, 0x1f, 0x00, 0x0f, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x0c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06,
    0x12, 0x04, 0x21, 0x00, 0x24, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x21,
    0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x22, 0x02, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x22, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x22, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x22, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x22, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12,
    0x03, 0x23, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x23,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x23, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x23, 0x12, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x23, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x07, 0x12, 0x04, 0x26, 0x00, 0x2b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12,
    0x03, 0x26, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x08, 0x00, 0x12, 0x04, 0x27, 0x02,
    0x2a, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x08, 0x00, 0x01, 0x12, 0x03, 0x27, 0x08, 0x0c,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x28, 0x04, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03, 0x28, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x28, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x28, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12,
    0x03, 0x29, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x06, 0x12, 0x03, 0x29,
    0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x29, 0x09, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x29, 0x10, 0x11, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x2d, 0x00, 0x32, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08,
    0x01, 0x12, 0x03, 0x2d, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x08, 0x00, 0x12, 0x04,
    0x2e, 0x02, 0x31, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x08, 0x00, 0x01, 0x12, 0x03, 0x2e,
    0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x2f, 0x04, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2f, 0x04, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2f, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2f, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02,
    0x01, 0x12, 0x03, 0x30, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x30, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x30,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x30, 0x14, 0x15,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x34, 0x00, 0x37, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x09, 0x01, 0x12, 0x03, 0x34, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00,
    0x12, 0x03, 0x35, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x35, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12, 0x03, 0x35, 0x0b,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x35, 0x19, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x35, 0x20, 0x21, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x36, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x36, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x36, 0x0b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x36, 0x19, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x36,
    0x20, 0x21, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x03, 0x39, 0x00, 0x14, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x39, 0x08, 0x11, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12,
    0x04, 0x3b, 0x00, 0x40, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x3b, 0x08,
    0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x08, 0x00, 0x12, 0x04, 0x3c, 0x02, 0x3f, 0x03, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x08, 0x00, 0x01, 0x12, 0x03, 0x3c, 0x08, 0x11, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x3d, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x3d, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x3d, 0x0e, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x3d, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x03, 0x3e, 0x04,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x06, 0x12, 0x03, 0x3e, 0x04, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3e, 0x12, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3e, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x0c, 0x12, 0x04, 0x42, 0x00, 0x44, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03,
    0x42, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x43, 0x02, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x03, 0x43, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x06, 0x12, 0x03, 0x43, 0x0b, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x43, 0x19, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x43, 0x27, 0x28,
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
