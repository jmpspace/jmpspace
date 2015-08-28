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
pub struct Unit {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Unit {
    pub fn new() -> Unit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Unit {
        static mut instance: ::protobuf::lazy::Lazy<Unit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Unit,
        };
        unsafe {
            instance.get(|| {
                Unit {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for Unit {
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
        ::std::any::TypeId::of::<Unit>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Unit {
    fn new() -> Unit {
        Unit::new()
    }

    fn descriptor_static(_: ::std::option::Option<Unit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Unit>(
                    "Unit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Unit {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Unit {
    fn eq(&self, other: &Unit) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Unit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Active {
    // message fields
    groups: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Active {
    pub fn new() -> Active {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Active {
        static mut instance: ::protobuf::lazy::Lazy<Active> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Active,
        };
        unsafe {
            instance.get(|| {
                Active {
                    groups: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated int32 groups = 1;

    pub fn clear_groups(&mut self) {
        self.groups.clear();
    }

    // Param is passed by value, moved
    pub fn set_groups(&mut self, v: ::std::vec::Vec<i32>) {
        self.groups = v;
    }

    // Mutable pointer to the field.
    pub fn mut_groups<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.groups
    }

    // Take field
    pub fn take_groups(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.groups, ::std::vec::Vec::new())
    }

    pub fn get_groups<'a>(&'a self) -> &'a [i32] {
        &self.groups
    }
}

impl ::protobuf::Message for Active {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.groups));
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
        for value in self.groups.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.groups.iter() {
            try!(os.write_int32(1, *v));
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
        ::std::any::TypeId::of::<Active>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Active {
    fn new() -> Active {
        Active::new()
    }

    fn descriptor_static(_: ::std::option::Option<Active>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "groups",
                    Active::get_groups,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Active>(
                    "Active",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Active {
    fn clear(&mut self) {
        self.clear_groups();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Active {
    fn eq(&self, other: &Active) -> bool {
        self.groups == other.groups &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Active {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Controls {
    // message fields
    // message oneof groups
    controls: ::std::option::Option<Controls_oneof_controls>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum Controls_oneof_controls {
    brakes(Unit),
    active(Active),
}

impl Controls {
    pub fn new() -> Controls {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Controls {
        static mut instance: ::protobuf::lazy::Lazy<Controls> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Controls,
        };
        unsafe {
            instance.get(|| {
                Controls {
                    controls: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .actions.Unit brakes = 1;

    pub fn clear_brakes(&mut self) {
        self.controls = ::std::option::Option::None;
    }

    pub fn has_brakes(&self) -> bool {
        match self.controls {
            ::std::option::Option::Some(Controls_oneof_controls::brakes(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_brakes(&mut self, v: Unit) {
        self.controls = ::std::option::Option::Some(Controls_oneof_controls::brakes(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_brakes<'a>(&'a mut self) -> &'a mut Unit {
        if let ::std::option::Option::Some(Controls_oneof_controls::brakes(_)) = self.controls {
        } else {
            self.controls = ::std::option::Option::Some(Controls_oneof_controls::brakes(Unit::new()));
        }
        match self.controls {
            ::std::option::Option::Some(Controls_oneof_controls::brakes(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_brakes(&mut self) -> Unit {
        if self.has_brakes() {
            match self.controls.take() {
                ::std::option::Option::Some(Controls_oneof_controls::brakes(v)) => v,
                _ => panic!(),
            }
        } else {
            Unit::new()
        }
    }

    pub fn get_brakes<'a>(&'a self) -> &'a Unit {
        match self.controls {
            ::std::option::Option::Some(Controls_oneof_controls::brakes(ref v)) => v,
            _ => Unit::default_instance(),
        }
    }

    // optional .actions.Active active = 2;

    pub fn clear_active(&mut self) {
        self.controls = ::std::option::Option::None;
    }

    pub fn has_active(&self) -> bool {
        match self.controls {
            ::std::option::Option::Some(Controls_oneof_controls::active(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_active(&mut self, v: Active) {
        self.controls = ::std::option::Option::Some(Controls_oneof_controls::active(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_active<'a>(&'a mut self) -> &'a mut Active {
        if let ::std::option::Option::Some(Controls_oneof_controls::active(_)) = self.controls {
        } else {
            self.controls = ::std::option::Option::Some(Controls_oneof_controls::active(Active::new()));
        }
        match self.controls {
            ::std::option::Option::Some(Controls_oneof_controls::active(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_active(&mut self) -> Active {
        if self.has_active() {
            match self.controls.take() {
                ::std::option::Option::Some(Controls_oneof_controls::active(v)) => v,
                _ => panic!(),
            }
        } else {
            Active::new()
        }
    }

    pub fn get_active<'a>(&'a self) -> &'a Active {
        match self.controls {
            ::std::option::Option::Some(Controls_oneof_controls::active(ref v)) => v,
            _ => Active::default_instance(),
        }
    }
}

impl ::protobuf::Message for Controls {
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
                    self.controls = ::std::option::Option::Some(Controls_oneof_controls::brakes(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.controls = ::std::option::Option::Some(Controls_oneof_controls::active(try!(is.read_message())));
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
        if let ::std::option::Option::Some(ref v) = self.controls {
            match v {
                &Controls_oneof_controls::brakes(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Controls_oneof_controls::active(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.controls {
            match v {
                &Controls_oneof_controls::brakes(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Controls_oneof_controls::active(ref v) => {
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
        ::std::any::TypeId::of::<Controls>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Controls {
    fn new() -> Controls {
        Controls::new()
    }

    fn descriptor_static(_: ::std::option::Option<Controls>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "brakes",
                    Controls::has_brakes,
                    Controls::get_brakes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "active",
                    Controls::has_active,
                    Controls::get_active,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Controls>(
                    "Controls",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Controls {
    fn clear(&mut self) {
        self.clear_brakes();
        self.clear_active();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Controls {
    fn eq(&self, other: &Controls) -> bool {
        self.controls == other.controls &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Controls {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Build {
    // message fields
    foo: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Build {
    pub fn new() -> Build {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Build {
        static mut instance: ::protobuf::lazy::Lazy<Build> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Build,
        };
        unsafe {
            instance.get(|| {
                Build {
                    foo: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 foo = 1;

    pub fn clear_foo(&mut self) {
        self.foo = ::std::option::Option::None;
    }

    pub fn has_foo(&self) -> bool {
        self.foo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_foo(&mut self, v: i32) {
        self.foo = ::std::option::Option::Some(v);
    }

    pub fn get_foo<'a>(&self) -> i32 {
        self.foo.unwrap_or(0)
    }
}

impl ::protobuf::Message for Build {
    fn is_initialized(&self) -> bool {
        if self.foo.is_none() {
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
                    self.foo = ::std::option::Option::Some(tmp);
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
        for value in self.foo.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.foo {
            try!(os.write_int32(1, v));
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
        ::std::any::TypeId::of::<Build>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Build {
    fn new() -> Build {
        Build::new()
    }

    fn descriptor_static(_: ::std::option::Option<Build>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "foo",
                    Build::has_foo,
                    Build::get_foo,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Build>(
                    "Build",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Build {
    fn clear(&mut self) {
        self.clear_foo();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Build {
    fn eq(&self, other: &Build) -> bool {
        self.foo == other.foo &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Build {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Action {
    // message fields
    // message oneof groups
    action: ::std::option::Option<Action_oneof_action>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum Action_oneof_action {
    controls(Controls),
    build(Build),
}

impl Action {
    pub fn new() -> Action {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Action {
        static mut instance: ::protobuf::lazy::Lazy<Action> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Action,
        };
        unsafe {
            instance.get(|| {
                Action {
                    action: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .actions.Controls controls = 1;

    pub fn clear_controls(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_controls(&self) -> bool {
        match self.action {
            ::std::option::Option::Some(Action_oneof_action::controls(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_controls(&mut self, v: Controls) {
        self.action = ::std::option::Option::Some(Action_oneof_action::controls(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_controls<'a>(&'a mut self) -> &'a mut Controls {
        if let ::std::option::Option::Some(Action_oneof_action::controls(_)) = self.action {
        } else {
            self.action = ::std::option::Option::Some(Action_oneof_action::controls(Controls::new()));
        }
        match self.action {
            ::std::option::Option::Some(Action_oneof_action::controls(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_controls(&mut self) -> Controls {
        if self.has_controls() {
            match self.action.take() {
                ::std::option::Option::Some(Action_oneof_action::controls(v)) => v,
                _ => panic!(),
            }
        } else {
            Controls::new()
        }
    }

    pub fn get_controls<'a>(&'a self) -> &'a Controls {
        match self.action {
            ::std::option::Option::Some(Action_oneof_action::controls(ref v)) => v,
            _ => Controls::default_instance(),
        }
    }

    // optional .actions.Build build = 2;

    pub fn clear_build(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_build(&self) -> bool {
        match self.action {
            ::std::option::Option::Some(Action_oneof_action::build(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_build(&mut self, v: Build) {
        self.action = ::std::option::Option::Some(Action_oneof_action::build(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_build<'a>(&'a mut self) -> &'a mut Build {
        if let ::std::option::Option::Some(Action_oneof_action::build(_)) = self.action {
        } else {
            self.action = ::std::option::Option::Some(Action_oneof_action::build(Build::new()));
        }
        match self.action {
            ::std::option::Option::Some(Action_oneof_action::build(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_build(&mut self) -> Build {
        if self.has_build() {
            match self.action.take() {
                ::std::option::Option::Some(Action_oneof_action::build(v)) => v,
                _ => panic!(),
            }
        } else {
            Build::new()
        }
    }

    pub fn get_build<'a>(&'a self) -> &'a Build {
        match self.action {
            ::std::option::Option::Some(Action_oneof_action::build(ref v)) => v,
            _ => Build::default_instance(),
        }
    }
}

impl ::protobuf::Message for Action {
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
                    self.action = ::std::option::Option::Some(Action_oneof_action::controls(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.action = ::std::option::Option::Some(Action_oneof_action::build(try!(is.read_message())));
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
        if let ::std::option::Option::Some(ref v) = self.action {
            match v {
                &Action_oneof_action::controls(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Action_oneof_action::build(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.action {
            match v {
                &Action_oneof_action::controls(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Action_oneof_action::build(ref v) => {
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
        ::std::any::TypeId::of::<Action>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Action {
    fn new() -> Action {
        Action::new()
    }

    fn descriptor_static(_: ::std::option::Option<Action>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "controls",
                    Action::has_controls,
                    Action::get_controls,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "build",
                    Action::has_build,
                    Action::get_build,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Action>(
                    "Action",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Action {
    fn clear(&mut self) {
        self.clear_controls();
        self.clear_build();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Action {
    fn eq(&self, other: &Action) -> bool {
        self.action == other.action &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x11, 0x73, 0x72, 0x63, 0x2f, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x07, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x22, 0x06, 0x0a, 0x04,
    0x55, 0x6e, 0x69, 0x74, 0x22, 0x18, 0x0a, 0x06, 0x41, 0x63, 0x74, 0x69, 0x76, 0x65, 0x12, 0x0e,
    0x0a, 0x06, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x05, 0x22, 0x5a,
    0x0a, 0x08, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x73, 0x12, 0x1f, 0x0a, 0x06, 0x62, 0x72,
    0x61, 0x6b, 0x65, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x55, 0x6e, 0x69, 0x74, 0x48, 0x00, 0x12, 0x21, 0x0a, 0x06, 0x61,
    0x63, 0x74, 0x69, 0x76, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x61, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x41, 0x63, 0x74, 0x69, 0x76, 0x65, 0x48, 0x00, 0x42, 0x0a,
    0x0a, 0x08, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x73, 0x22, 0x14, 0x0a, 0x05, 0x42, 0x75,
    0x69, 0x6c, 0x64, 0x12, 0x0b, 0x0a, 0x03, 0x66, 0x6f, 0x6f, 0x18, 0x01, 0x20, 0x02, 0x28, 0x05,
    0x22, 0x5a, 0x0a, 0x06, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x25, 0x0a, 0x08, 0x63, 0x6f,
    0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x61,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x73, 0x48,
    0x00, 0x12, 0x1f, 0x0a, 0x05, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x0e, 0x2e, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x42, 0x75, 0x69, 0x6c, 0x64,
    0x48, 0x00, 0x42, 0x08, 0x0a, 0x06, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x4a, 0xa7, 0x04, 0x0a,
    0x06, 0x12, 0x04, 0x01, 0x00, 0x19, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08,
    0x0f, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x03, 0x03, 0x00, 0x0f, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x0c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x05, 0x00, 0x07, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x05, 0x08, 0x0e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x06, 0x02, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x06, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x06, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x06, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x06, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x09, 0x00, 0x0e,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x09, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x02, 0x08, 0x00, 0x12, 0x04, 0x0a, 0x02, 0x0d, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x08, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x00, 0x12, 0x03, 0x0b, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x0b, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b,
    0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x12, 0x13,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x04, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0c, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x0c, 0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x10,
    0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x10, 0x08, 0x0d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x11, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x11, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x11, 0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x11, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x14, 0x00, 0x19, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x14, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x04, 0x08, 0x00, 0x12, 0x04, 0x15, 0x02, 0x18, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x08, 0x00, 0x01, 0x12, 0x03, 0x15, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00,
    0x12, 0x03, 0x16, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x16, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x0d,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16, 0x18, 0x19, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x17, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x17, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x17, 0x0a, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x17, 0x12, 0x13,
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
