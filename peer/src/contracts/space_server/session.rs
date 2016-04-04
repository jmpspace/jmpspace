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
pub struct AuthCredential {
    // message fields
    username: ::protobuf::SingularField<::std::string::String>,
    password: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl AuthCredential {
    pub fn new() -> AuthCredential {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthCredential {
        static mut instance: ::protobuf::lazy::Lazy<AuthCredential> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthCredential,
        };
        unsafe {
            instance.get(|| {
                AuthCredential {
                    username: ::protobuf::SingularField::none(),
                    password: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string username = 1;

    pub fn clear_username(&mut self) {
        self.username.clear();
    }

    pub fn has_username(&self) -> bool {
        self.username.is_some()
    }

    // Param is passed by value, moved
    pub fn set_username(&mut self, v: ::std::string::String) {
        self.username = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_username<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.username.is_none() {
            self.username.set_default();
        };
        self.username.as_mut().unwrap()
    }

    // Take field
    pub fn take_username(&mut self) -> ::std::string::String {
        self.username.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_username<'a>(&'a self) -> &'a str {
        match self.username.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string password = 2;

    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    pub fn has_password(&self) -> bool {
        self.password.is_some()
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::string::String) {
        self.password = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.password.is_none() {
            self.password.set_default();
        };
        self.password.as_mut().unwrap()
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::string::String {
        self.password.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_password<'a>(&'a self) -> &'a str {
        match self.password.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for AuthCredential {
    fn is_initialized(&self) -> bool {
        if self.username.is_none() {
            return false;
        };
        if self.password.is_none() {
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
                    let tmp = self.username.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.password.set_default();
                    try!(is.read_string_into(tmp))
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
        for value in self.username.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.password.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.username.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.password.as_ref() {
            try!(os.write_string(2, &v));
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
        ::std::any::TypeId::of::<AuthCredential>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AuthCredential {
    fn new() -> AuthCredential {
        AuthCredential::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthCredential>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "username",
                    AuthCredential::has_username,
                    AuthCredential::get_username,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "password",
                    AuthCredential::has_password,
                    AuthCredential::get_password,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthCredential>(
                    "AuthCredential",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthCredential {
    fn clear(&mut self) {
        self.clear_username();
        self.clear_password();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AuthCredential {
    fn eq(&self, other: &AuthCredential) -> bool {
        self.username == other.username &&
        self.password == other.password &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AuthCredential {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AuthRequest {
    // message fields
    credential: ::protobuf::SingularPtrField<AuthCredential>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl AuthRequest {
    pub fn new() -> AuthRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthRequest {
        static mut instance: ::protobuf::lazy::Lazy<AuthRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthRequest,
        };
        unsafe {
            instance.get(|| {
                AuthRequest {
                    credential: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .spaceServer.session.AuthCredential credential = 1;

    pub fn clear_credential(&mut self) {
        self.credential.clear();
    }

    pub fn has_credential(&self) -> bool {
        self.credential.is_some()
    }

    // Param is passed by value, moved
    pub fn set_credential(&mut self, v: AuthCredential) {
        self.credential = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_credential<'a>(&'a mut self) -> &'a mut AuthCredential {
        if self.credential.is_none() {
            self.credential.set_default();
        };
        self.credential.as_mut().unwrap()
    }

    // Take field
    pub fn take_credential(&mut self) -> AuthCredential {
        self.credential.take().unwrap_or_else(|| AuthCredential::new())
    }

    pub fn get_credential<'a>(&'a self) -> &'a AuthCredential {
        self.credential.as_ref().unwrap_or_else(|| AuthCredential::default_instance())
    }
}

impl ::protobuf::Message for AuthRequest {
    fn is_initialized(&self) -> bool {
        if self.credential.is_none() {
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
                    let tmp = self.credential.set_default();
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
        for value in self.credential.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.credential.as_ref() {
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
        ::std::any::TypeId::of::<AuthRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AuthRequest {
    fn new() -> AuthRequest {
        AuthRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "credential",
                    AuthRequest::has_credential,
                    AuthRequest::get_credential,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthRequest>(
                    "AuthRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthRequest {
    fn clear(&mut self) {
        self.clear_credential();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AuthRequest {
    fn eq(&self, other: &AuthRequest) -> bool {
        self.credential == other.credential &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AuthRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AuthFailure {
    // message fields
    error: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl AuthFailure {
    pub fn new() -> AuthFailure {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthFailure {
        static mut instance: ::protobuf::lazy::Lazy<AuthFailure> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthFailure,
        };
        unsafe {
            instance.get(|| {
                AuthFailure {
                    error: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        self.error.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error<'a>(&'a self) -> &'a str {
        match self.error.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for AuthFailure {
    fn is_initialized(&self) -> bool {
        if self.error.is_none() {
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
                    let tmp = self.error.set_default();
                    try!(is.read_string_into(tmp))
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
        for value in self.error.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            try!(os.write_string(1, &v));
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
        ::std::any::TypeId::of::<AuthFailure>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AuthFailure {
    fn new() -> AuthFailure {
        AuthFailure::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthFailure>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "error",
                    AuthFailure::has_error,
                    AuthFailure::get_error,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthFailure>(
                    "AuthFailure",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthFailure {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AuthFailure {
    fn eq(&self, other: &AuthFailure) -> bool {
        self.error == other.error &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AuthFailure {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AuthSuccess {
    // message fields
    session: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl AuthSuccess {
    pub fn new() -> AuthSuccess {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthSuccess {
        static mut instance: ::protobuf::lazy::Lazy<AuthSuccess> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthSuccess,
        };
        unsafe {
            instance.get(|| {
                AuthSuccess {
                    session: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 session = 1;

    pub fn clear_session(&mut self) {
        self.session = ::std::option::Option::None;
    }

    pub fn has_session(&self) -> bool {
        self.session.is_some()
    }

    // Param is passed by value, moved
    pub fn set_session(&mut self, v: i32) {
        self.session = ::std::option::Option::Some(v);
    }

    pub fn get_session<'a>(&self) -> i32 {
        self.session.unwrap_or(0)
    }
}

impl ::protobuf::Message for AuthSuccess {
    fn is_initialized(&self) -> bool {
        if self.session.is_none() {
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
                    self.session = ::std::option::Option::Some(tmp);
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
        for value in self.session.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.session {
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
        ::std::any::TypeId::of::<AuthSuccess>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AuthSuccess {
    fn new() -> AuthSuccess {
        AuthSuccess::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthSuccess>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "session",
                    AuthSuccess::has_session,
                    AuthSuccess::get_session,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthSuccess>(
                    "AuthSuccess",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthSuccess {
    fn clear(&mut self) {
        self.clear_session();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AuthSuccess {
    fn eq(&self, other: &AuthSuccess) -> bool {
        self.session == other.session &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AuthSuccess {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AuthResponse {
    // message fields
    // message oneof groups
    response: ::std::option::Option<AuthResponse_oneof_response>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum AuthResponse_oneof_response {
    failure(AuthFailure),
    success(AuthSuccess),
}

impl AuthResponse {
    pub fn new() -> AuthResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthResponse {
        static mut instance: ::protobuf::lazy::Lazy<AuthResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthResponse,
        };
        unsafe {
            instance.get(|| {
                AuthResponse {
                    response: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .spaceServer.session.AuthFailure failure = 1;

    pub fn clear_failure(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_failure(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(AuthResponse_oneof_response::failure(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_failure(&mut self, v: AuthFailure) {
        self.response = ::std::option::Option::Some(AuthResponse_oneof_response::failure(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_failure<'a>(&'a mut self) -> &'a mut AuthFailure {
        if let ::std::option::Option::Some(AuthResponse_oneof_response::failure(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(AuthResponse_oneof_response::failure(AuthFailure::new()));
        }
        match self.response {
            ::std::option::Option::Some(AuthResponse_oneof_response::failure(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_failure(&mut self) -> AuthFailure {
        if self.has_failure() {
            match self.response.take() {
                ::std::option::Option::Some(AuthResponse_oneof_response::failure(v)) => v,
                _ => panic!(),
            }
        } else {
            AuthFailure::new()
        }
    }

    pub fn get_failure<'a>(&'a self) -> &'a AuthFailure {
        match self.response {
            ::std::option::Option::Some(AuthResponse_oneof_response::failure(ref v)) => v,
            _ => AuthFailure::default_instance(),
        }
    }

    // optional .spaceServer.session.AuthSuccess success = 2;

    pub fn clear_success(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(AuthResponse_oneof_response::success(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: AuthSuccess) {
        self.response = ::std::option::Option::Some(AuthResponse_oneof_response::success(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_success<'a>(&'a mut self) -> &'a mut AuthSuccess {
        if let ::std::option::Option::Some(AuthResponse_oneof_response::success(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(AuthResponse_oneof_response::success(AuthSuccess::new()));
        }
        match self.response {
            ::std::option::Option::Some(AuthResponse_oneof_response::success(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_success(&mut self) -> AuthSuccess {
        if self.has_success() {
            match self.response.take() {
                ::std::option::Option::Some(AuthResponse_oneof_response::success(v)) => v,
                _ => panic!(),
            }
        } else {
            AuthSuccess::new()
        }
    }

    pub fn get_success<'a>(&'a self) -> &'a AuthSuccess {
        match self.response {
            ::std::option::Option::Some(AuthResponse_oneof_response::success(ref v)) => v,
            _ => AuthSuccess::default_instance(),
        }
    }
}

impl ::protobuf::Message for AuthResponse {
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
                    self.response = ::std::option::Option::Some(AuthResponse_oneof_response::failure(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.response = ::std::option::Option::Some(AuthResponse_oneof_response::success(try!(is.read_message())));
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
        if let ::std::option::Option::Some(ref v) = self.response {
            match v {
                &AuthResponse_oneof_response::failure(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &AuthResponse_oneof_response::success(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.response {
            match v {
                &AuthResponse_oneof_response::failure(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &AuthResponse_oneof_response::success(ref v) => {
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
        ::std::any::TypeId::of::<AuthResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AuthResponse {
    fn new() -> AuthResponse {
        AuthResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "failure",
                    AuthResponse::has_failure,
                    AuthResponse::get_failure,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "success",
                    AuthResponse::has_success,
                    AuthResponse::get_success,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthResponse>(
                    "AuthResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthResponse {
    fn clear(&mut self) {
        self.clear_failure();
        self.clear_success();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AuthResponse {
    fn eq(&self, other: &AuthResponse) -> bool {
        self.response == other.response &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AuthResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x13, 0x73, 0x70, 0x61, 0x63, 0x65, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x73, 0x65, 0x73,
    0x73, 0x69, 0x6f, 0x6e, 0x22, 0x34, 0x0a, 0x0e, 0x41, 0x75, 0x74, 0x68, 0x43, 0x72, 0x65, 0x64,
    0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x12, 0x10, 0x0a, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x70, 0x61, 0x73, 0x73,
    0x77, 0x6f, 0x72, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x22, 0x46, 0x0a, 0x0b, 0x41, 0x75,
    0x74, 0x68, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x37, 0x0a, 0x0a, 0x63, 0x72, 0x65,
    0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x23, 0x2e,
    0x73, 0x70, 0x61, 0x63, 0x65, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x73, 0x65, 0x73, 0x73,
    0x69, 0x6f, 0x6e, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x43, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69,
    0x61, 0x6c, 0x22, 0x1c, 0x0a, 0x0b, 0x41, 0x75, 0x74, 0x68, 0x46, 0x61, 0x69, 0x6c, 0x75, 0x72,
    0x65, 0x12, 0x0d, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09,
    0x22, 0x1e, 0x0a, 0x0b, 0x41, 0x75, 0x74, 0x68, 0x53, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x12,
    0x0f, 0x0a, 0x07, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x02, 0x28, 0x05,
    0x22, 0x84, 0x01, 0x0a, 0x0c, 0x41, 0x75, 0x74, 0x68, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x12, 0x33, 0x0a, 0x07, 0x66, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x20, 0x2e, 0x73, 0x70, 0x61, 0x63, 0x65, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x2e, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x46, 0x61, 0x69,
    0x6c, 0x75, 0x72, 0x65, 0x48, 0x00, 0x12, 0x33, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73,
    0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x73, 0x70, 0x61, 0x63, 0x65, 0x53,
    0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x41, 0x75,
    0x74, 0x68, 0x53, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x48, 0x00, 0x42, 0x0a, 0x0a, 0x08, 0x72,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x4a, 0xed, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x01, 0x00,
    0x1b, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x03, 0x08, 0x1b, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x05, 0x00, 0x08, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x05, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x02,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x06, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x06, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x06, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x07, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x07, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x07,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x07, 0x12, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x07, 0x1d, 0x1e, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0a, 0x00, 0x0c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x0a, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x0b, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0b, 0x0b, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x1a, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x02, 0x12, 0x04, 0x0e, 0x00, 0x10, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03,
    0x0e, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x02, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x0f, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04,
    0x12, 0x00, 0x14, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x12, 0x08, 0x13,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x13, 0x02, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x13, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x13, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x13, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x16, 0x00, 0x1b,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x16, 0x08, 0x14, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x04, 0x08, 0x00, 0x12, 0x04, 0x17, 0x02, 0x1a, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x08, 0x00, 0x01, 0x12, 0x03, 0x17, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x00, 0x12, 0x03, 0x18, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x18, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18,
    0x10, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x1a, 0x1b,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x19, 0x04, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x19, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x19, 0x10, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x19, 0x1a, 0x1b,
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
