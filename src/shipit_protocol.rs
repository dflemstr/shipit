// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Request {
    // message fields
    access_token: ::protobuf::SingularField<::std::string::String>,
    // message oneof groups
    msg: ::std::option::Option<Request_oneof_msg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum Request_oneof_msg {
    error(Error),
    identify(Identify),
    ping(Ping),
}

impl Request {
    pub fn new() -> Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Request {
        static mut instance: ::protobuf::lazy::Lazy<Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Request,
        };
        unsafe {
            instance.get(|| {
                Request {
                    access_token: ::protobuf::SingularField::none(),
                    msg: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .Error error = 1;

    pub fn clear_error(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(Request_oneof_msg::error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: Error) {
        self.msg = ::std::option::Option::Some(Request_oneof_msg::error(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error<'a>(&'a mut self) -> &'a mut Error {
        if let ::std::option::Option::Some(Request_oneof_msg::error(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(Request_oneof_msg::error(Error::new()));
        }
        match self.msg {
            ::std::option::Option::Some(Request_oneof_msg::error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> Error {
        if self.has_error() {
            match self.msg.take() {
                ::std::option::Option::Some(Request_oneof_msg::error(v)) => v,
                _ => panic!(),
            }
        } else {
            Error::new()
        }
    }

    pub fn get_error<'a>(&'a self) -> &'a Error {
        match self.msg {
            ::std::option::Option::Some(Request_oneof_msg::error(ref v)) => v,
            _ => Error::default_instance(),
        }
    }

    // optional .Identify identify = 2;

    pub fn clear_identify(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_identify(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(Request_oneof_msg::identify(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_identify(&mut self, v: Identify) {
        self.msg = ::std::option::Option::Some(Request_oneof_msg::identify(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_identify<'a>(&'a mut self) -> &'a mut Identify {
        if let ::std::option::Option::Some(Request_oneof_msg::identify(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(Request_oneof_msg::identify(Identify::new()));
        }
        match self.msg {
            ::std::option::Option::Some(Request_oneof_msg::identify(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_identify(&mut self) -> Identify {
        if self.has_identify() {
            match self.msg.take() {
                ::std::option::Option::Some(Request_oneof_msg::identify(v)) => v,
                _ => panic!(),
            }
        } else {
            Identify::new()
        }
    }

    pub fn get_identify<'a>(&'a self) -> &'a Identify {
        match self.msg {
            ::std::option::Option::Some(Request_oneof_msg::identify(ref v)) => v,
            _ => Identify::default_instance(),
        }
    }

    // optional .Ping ping = 3;

    pub fn clear_ping(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_ping(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(Request_oneof_msg::ping(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ping(&mut self, v: Ping) {
        self.msg = ::std::option::Option::Some(Request_oneof_msg::ping(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ping<'a>(&'a mut self) -> &'a mut Ping {
        if let ::std::option::Option::Some(Request_oneof_msg::ping(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(Request_oneof_msg::ping(Ping::new()));
        }
        match self.msg {
            ::std::option::Option::Some(Request_oneof_msg::ping(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ping(&mut self) -> Ping {
        if self.has_ping() {
            match self.msg.take() {
                ::std::option::Option::Some(Request_oneof_msg::ping(v)) => v,
                _ => panic!(),
            }
        } else {
            Ping::new()
        }
    }

    pub fn get_ping<'a>(&'a self) -> &'a Ping {
        match self.msg {
            ::std::option::Option::Some(Request_oneof_msg::ping(ref v)) => v,
            _ => Ping::default_instance(),
        }
    }

    // optional string access_token = 64;

    pub fn clear_access_token(&mut self) {
        self.access_token.clear();
    }

    pub fn has_access_token(&self) -> bool {
        self.access_token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_access_token(&mut self, v: ::std::string::String) {
        self.access_token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_access_token<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.access_token.is_none() {
            self.access_token.set_default();
        };
        self.access_token.as_mut().unwrap()
    }

    // Take field
    pub fn take_access_token(&mut self) -> ::std::string::String {
        self.access_token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_access_token<'a>(&'a self) -> &'a str {
        match self.access_token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Request {
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
                    self.msg = ::std::option::Option::Some(Request_oneof_msg::error(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.msg = ::std::option::Option::Some(Request_oneof_msg::identify(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.msg = ::std::option::Option::Some(Request_oneof_msg::ping(try!(is.read_message())));
                },
                64 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.access_token.set_default();
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
        for value in self.access_token.iter() {
            my_size += ::protobuf::rt::string_size(64, &value);
        };
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &Request_oneof_msg::error(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_msg::identify(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_msg::ping(ref v) => {
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
        if let Some(v) = self.access_token.as_ref() {
            try!(os.write_string(64, &v));
        };
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &Request_oneof_msg::error(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Request_oneof_msg::identify(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Request_oneof_msg::ping(ref v) => {
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
        ::std::any::TypeId::of::<Request>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Request {
    fn new() -> Request {
        Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    Request::has_error,
                    Request::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "identify",
                    Request::has_identify,
                    Request::get_identify,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "ping",
                    Request::has_ping,
                    Request::get_ping,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "access_token",
                    Request::has_access_token,
                    Request::get_access_token,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Request>(
                    "Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_identify();
        self.clear_ping();
        self.clear_access_token();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Request {
    fn eq(&self, other: &Request) -> bool {
        self.access_token == other.access_token &&
        self.msg == other.msg &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Identify {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Identify {
    pub fn new() -> Identify {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Identify {
        static mut instance: ::protobuf::lazy::Lazy<Identify> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Identify,
        };
        unsafe {
            instance.get(|| {
                Identify {
                    name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Identify {
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
                    let tmp = self.name.set_default();
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
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
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
        ::std::any::TypeId::of::<Identify>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Identify {
    fn new() -> Identify {
        Identify::new()
    }

    fn descriptor_static(_: ::std::option::Option<Identify>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Identify::has_name,
                    Identify::get_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Identify>(
                    "Identify",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Identify {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Identify {
    fn eq(&self, other: &Identify) -> bool {
        self.name == other.name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Identify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Ping {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Ping {
    pub fn new() -> Ping {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Ping {
        static mut instance: ::protobuf::lazy::Lazy<Ping> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Ping,
        };
        unsafe {
            instance.get(|| {
                Ping {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for Ping {
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
        ::std::any::TypeId::of::<Ping>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Ping {
    fn new() -> Ping {
        Ping::new()
    }

    fn descriptor_static(_: ::std::option::Option<Ping>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Ping>(
                    "Ping",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Ping {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Ping {
    fn eq(&self, other: &Ping) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Ping {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Response {
    // message fields
    // message oneof groups
    msg: ::std::option::Option<Response_oneof_msg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum Response_oneof_msg {
    error(Error),
    identified(Identified),
    pong(Pong),
}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Response {
        static mut instance: ::protobuf::lazy::Lazy<Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Response,
        };
        unsafe {
            instance.get(|| {
                Response {
                    msg: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .Error error = 1;

    pub fn clear_error(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(Response_oneof_msg::error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: Error) {
        self.msg = ::std::option::Option::Some(Response_oneof_msg::error(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error<'a>(&'a mut self) -> &'a mut Error {
        if let ::std::option::Option::Some(Response_oneof_msg::error(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(Response_oneof_msg::error(Error::new()));
        }
        match self.msg {
            ::std::option::Option::Some(Response_oneof_msg::error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> Error {
        if self.has_error() {
            match self.msg.take() {
                ::std::option::Option::Some(Response_oneof_msg::error(v)) => v,
                _ => panic!(),
            }
        } else {
            Error::new()
        }
    }

    pub fn get_error<'a>(&'a self) -> &'a Error {
        match self.msg {
            ::std::option::Option::Some(Response_oneof_msg::error(ref v)) => v,
            _ => Error::default_instance(),
        }
    }

    // optional .Identified identified = 2;

    pub fn clear_identified(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_identified(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(Response_oneof_msg::identified(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_identified(&mut self, v: Identified) {
        self.msg = ::std::option::Option::Some(Response_oneof_msg::identified(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_identified<'a>(&'a mut self) -> &'a mut Identified {
        if let ::std::option::Option::Some(Response_oneof_msg::identified(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(Response_oneof_msg::identified(Identified::new()));
        }
        match self.msg {
            ::std::option::Option::Some(Response_oneof_msg::identified(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_identified(&mut self) -> Identified {
        if self.has_identified() {
            match self.msg.take() {
                ::std::option::Option::Some(Response_oneof_msg::identified(v)) => v,
                _ => panic!(),
            }
        } else {
            Identified::new()
        }
    }

    pub fn get_identified<'a>(&'a self) -> &'a Identified {
        match self.msg {
            ::std::option::Option::Some(Response_oneof_msg::identified(ref v)) => v,
            _ => Identified::default_instance(),
        }
    }

    // optional .Pong pong = 3;

    pub fn clear_pong(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_pong(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(Response_oneof_msg::pong(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_pong(&mut self, v: Pong) {
        self.msg = ::std::option::Option::Some(Response_oneof_msg::pong(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pong<'a>(&'a mut self) -> &'a mut Pong {
        if let ::std::option::Option::Some(Response_oneof_msg::pong(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(Response_oneof_msg::pong(Pong::new()));
        }
        match self.msg {
            ::std::option::Option::Some(Response_oneof_msg::pong(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_pong(&mut self) -> Pong {
        if self.has_pong() {
            match self.msg.take() {
                ::std::option::Option::Some(Response_oneof_msg::pong(v)) => v,
                _ => panic!(),
            }
        } else {
            Pong::new()
        }
    }

    pub fn get_pong<'a>(&'a self) -> &'a Pong {
        match self.msg {
            ::std::option::Option::Some(Response_oneof_msg::pong(ref v)) => v,
            _ => Pong::default_instance(),
        }
    }
}

impl ::protobuf::Message for Response {
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
                    self.msg = ::std::option::Option::Some(Response_oneof_msg::error(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.msg = ::std::option::Option::Some(Response_oneof_msg::identified(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.msg = ::std::option::Option::Some(Response_oneof_msg::pong(try!(is.read_message())));
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
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &Response_oneof_msg::error(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_msg::identified(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_msg::pong(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &Response_oneof_msg::error(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Response_oneof_msg::identified(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Response_oneof_msg::pong(ref v) => {
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
        ::std::any::TypeId::of::<Response>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Response {
    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    Response::has_error,
                    Response::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "identified",
                    Response::has_identified,
                    Response::get_identified,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pong",
                    Response::has_pong,
                    Response::get_pong,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Response>(
                    "Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_identified();
        self.clear_pong();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Response {
    fn eq(&self, other: &Response) -> bool {
        self.msg == other.msg &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Identified {
    // message fields
    access_token: ::protobuf::SingularField<::std::string::String>,
    server_info: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Identified {
    pub fn new() -> Identified {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Identified {
        static mut instance: ::protobuf::lazy::Lazy<Identified> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Identified,
        };
        unsafe {
            instance.get(|| {
                Identified {
                    access_token: ::protobuf::SingularField::none(),
                    server_info: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string access_token = 1;

    pub fn clear_access_token(&mut self) {
        self.access_token.clear();
    }

    pub fn has_access_token(&self) -> bool {
        self.access_token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_access_token(&mut self, v: ::std::string::String) {
        self.access_token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_access_token<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.access_token.is_none() {
            self.access_token.set_default();
        };
        self.access_token.as_mut().unwrap()
    }

    // Take field
    pub fn take_access_token(&mut self) -> ::std::string::String {
        self.access_token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_access_token<'a>(&'a self) -> &'a str {
        match self.access_token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string server_info = 2;

    pub fn clear_server_info(&mut self) {
        self.server_info.clear();
    }

    pub fn has_server_info(&self) -> bool {
        self.server_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_info(&mut self, v: ::std::string::String) {
        self.server_info = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_server_info<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.server_info.is_none() {
            self.server_info.set_default();
        };
        self.server_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_server_info(&mut self) -> ::std::string::String {
        self.server_info.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_server_info<'a>(&'a self) -> &'a str {
        match self.server_info.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Identified {
    fn is_initialized(&self) -> bool {
        if self.access_token.is_none() {
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
                    let tmp = self.access_token.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.server_info.set_default();
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
        for value in self.access_token.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.server_info.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.access_token.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.server_info.as_ref() {
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
        ::std::any::TypeId::of::<Identified>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Identified {
    fn new() -> Identified {
        Identified::new()
    }

    fn descriptor_static(_: ::std::option::Option<Identified>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "access_token",
                    Identified::has_access_token,
                    Identified::get_access_token,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "server_info",
                    Identified::has_server_info,
                    Identified::get_server_info,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Identified>(
                    "Identified",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Identified {
    fn clear(&mut self) {
        self.clear_access_token();
        self.clear_server_info();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Identified {
    fn eq(&self, other: &Identified) -> bool {
        self.access_token == other.access_token &&
        self.server_info == other.server_info &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Identified {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Pong {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Pong {
    pub fn new() -> Pong {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Pong {
        static mut instance: ::protobuf::lazy::Lazy<Pong> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Pong,
        };
        unsafe {
            instance.get(|| {
                Pong {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for Pong {
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
        ::std::any::TypeId::of::<Pong>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Pong {
    fn new() -> Pong {
        Pong::new()
    }

    fn descriptor_static(_: ::std::option::Option<Pong>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Pong>(
                    "Pong",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Pong {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Pong {
    fn eq(&self, other: &Pong) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Pong {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Error {
    // message fields
    kind: ::std::option::Option<Error_Kind>,
    msg: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Error {
    pub fn new() -> Error {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Error {
        static mut instance: ::protobuf::lazy::Lazy<Error> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Error,
        };
        unsafe {
            instance.get(|| {
                Error {
                    kind: ::std::option::Option::None,
                    msg: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .Error.Kind kind = 1;

    pub fn clear_kind(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_kind(&self) -> bool {
        self.kind.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kind(&mut self, v: Error_Kind) {
        self.kind = ::std::option::Option::Some(v);
    }

    pub fn get_kind<'a>(&self) -> Error_Kind {
        self.kind.unwrap_or(Error_Kind::WIRE_ERROR)
    }

    // optional string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.msg.is_none() {
            self.msg.set_default();
        };
        self.msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        self.msg.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_msg<'a>(&'a self) -> &'a str {
        match self.msg.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Error {
    fn is_initialized(&self) -> bool {
        if self.kind.is_none() {
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
                    let tmp = try!(is.read_enum());
                    self.kind = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.msg.set_default();
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
        for value in self.kind.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.msg.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.kind {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.msg.as_ref() {
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
        ::std::any::TypeId::of::<Error>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Error {
    fn new() -> Error {
        Error::new()
    }

    fn descriptor_static(_: ::std::option::Option<Error>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "kind",
                    Error::has_kind,
                    Error::get_kind,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "msg",
                    Error::has_msg,
                    Error::get_msg,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Error>(
                    "Error",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Error {
    fn clear(&mut self) {
        self.clear_kind();
        self.clear_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        self.kind == other.kind &&
        self.msg == other.msg &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum Error_Kind {
    WIRE_ERROR = 1,
    IO_ERROR = 2,
    UNKNOWN_REQUEST = 3,
    UNAUTHORIZED = 4,
    PLAYER_NAME_TAKEN = 5,
}

impl ::protobuf::ProtobufEnum for Error_Kind {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Error_Kind> {
        match value {
            1 => ::std::option::Option::Some(Error_Kind::WIRE_ERROR),
            2 => ::std::option::Option::Some(Error_Kind::IO_ERROR),
            3 => ::std::option::Option::Some(Error_Kind::UNKNOWN_REQUEST),
            4 => ::std::option::Option::Some(Error_Kind::UNAUTHORIZED),
            5 => ::std::option::Option::Some(Error_Kind::PLAYER_NAME_TAKEN),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<Error_Kind>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Error_Kind", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Error_Kind {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x19, 0x73, 0x72, 0x63, 0x2f, 0x73, 0x68, 0x69, 0x70, 0x69, 0x74, 0x5f, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x75, 0x0a, 0x07, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x17, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x48, 0x00, 0x12,
    0x1d, 0x0a, 0x08, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x09, 0x2e, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x79, 0x48, 0x00, 0x12, 0x15,
    0x0a, 0x04, 0x70, 0x69, 0x6e, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x50,
    0x69, 0x6e, 0x67, 0x48, 0x00, 0x12, 0x14, 0x0a, 0x0c, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x5f,
    0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x40, 0x20, 0x01, 0x28, 0x09, 0x42, 0x05, 0x0a, 0x03, 0x6d,
    0x73, 0x67, 0x22, 0x18, 0x0a, 0x08, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x79, 0x12, 0x0c,
    0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x22, 0x06, 0x0a, 0x04,
    0x50, 0x69, 0x6e, 0x67, 0x22, 0x64, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x12, 0x17, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x06, 0x2e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x48, 0x00, 0x12, 0x21, 0x0a, 0x0a, 0x69, 0x64, 0x65,
    0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e,
    0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x64, 0x48, 0x00, 0x12, 0x15, 0x0a, 0x04,
    0x70, 0x6f, 0x6e, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x50, 0x6f, 0x6e,
    0x67, 0x48, 0x00, 0x42, 0x05, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x22, 0x37, 0x0a, 0x0a, 0x49, 0x64,
    0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x64, 0x12, 0x14, 0x0a, 0x0c, 0x61, 0x63, 0x63, 0x65,
    0x73, 0x73, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x13,
    0x0a, 0x0b, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x09, 0x22, 0x06, 0x0a, 0x04, 0x50, 0x6f, 0x6e, 0x67, 0x22, 0x93, 0x01, 0x0a, 0x05,
    0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x19, 0x0a, 0x04, 0x6b, 0x69, 0x6e, 0x64, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0e, 0x32, 0x0b, 0x2e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x2e, 0x4b, 0x69, 0x6e, 0x64,
    0x12, 0x0b, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x22, 0x62, 0x0a,
    0x04, 0x4b, 0x69, 0x6e, 0x64, 0x12, 0x0e, 0x0a, 0x0a, 0x57, 0x49, 0x52, 0x45, 0x5f, 0x45, 0x52,
    0x52, 0x4f, 0x52, 0x10, 0x01, 0x12, 0x0c, 0x0a, 0x08, 0x49, 0x4f, 0x5f, 0x45, 0x52, 0x52, 0x4f,
    0x52, 0x10, 0x02, 0x12, 0x13, 0x0a, 0x0f, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x5f, 0x52,
    0x45, 0x51, 0x55, 0x45, 0x53, 0x54, 0x10, 0x03, 0x12, 0x10, 0x0a, 0x0c, 0x55, 0x4e, 0x41, 0x55,
    0x54, 0x48, 0x4f, 0x52, 0x49, 0x5a, 0x45, 0x44, 0x10, 0x04, 0x12, 0x15, 0x0a, 0x11, 0x50, 0x4c,
    0x41, 0x59, 0x45, 0x52, 0x5f, 0x4e, 0x41, 0x4d, 0x45, 0x5f, 0x54, 0x41, 0x4b, 0x45, 0x4e, 0x10,
    0x05, 0x4a, 0xf0, 0x09, 0x0a, 0x06, 0x12, 0x04, 0x01, 0x00, 0x2d, 0x01, 0x0a, 0x16, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x01, 0x00, 0x08, 0x01, 0x1a, 0x0a, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x01, 0x08, 0x0f,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x08, 0x00, 0x12, 0x04, 0x02, 0x02, 0x06, 0x03, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x08, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08, 0x0b, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x03, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x03, 0x0a, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x03, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x04, 0x04, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x04, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x04, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x05, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06,
    0x12, 0x03, 0x05, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x05, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x05, 0x10,
    0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x07, 0x02, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x07, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x07, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x07, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x07, 0x21, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0a, 0x00,
    0x0c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x10, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x0b, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x0b, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0e, 0x00, 0x0f, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x0c, 0x0a, 0x17, 0x0a, 0x02, 0x04,
    0x03, 0x12, 0x04, 0x12, 0x00, 0x18, 0x01, 0x1a, 0x0b, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x12, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x08, 0x00, 0x12, 0x04, 0x13, 0x02, 0x17, 0x03, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x08, 0x00, 0x01, 0x12, 0x03, 0x13, 0x08, 0x0b, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x14, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x14, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x14, 0x0a, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x14, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x15, 0x04, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x15, 0x04, 0x0e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x15, 0x0f, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x15, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x02, 0x12, 0x03, 0x16, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x06,
    0x12, 0x03, 0x16, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x16, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x16, 0x10,
    0x11, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x1a, 0x00, 0x1d, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x1a, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x00, 0x12, 0x03, 0x1b, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x1b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1b,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x12, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1b, 0x21, 0x22, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x1c, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x1c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x1c, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x1c, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x1f, 0x00, 0x20, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x0c, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x06, 0x12, 0x04, 0x22, 0x00, 0x2d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03,
    0x22, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x04, 0x00, 0x12, 0x04, 0x23, 0x02, 0x29,
    0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x04, 0x00, 0x01, 0x12, 0x03, 0x23, 0x07, 0x0b, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x06, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x24, 0x04, 0x13, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x04, 0x0e, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x24, 0x11, 0x12, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x06, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x25, 0x04, 0x11, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x25, 0x04, 0x0c, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x25, 0x0f, 0x10, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x06, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x26, 0x04, 0x18, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x06, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x26, 0x04, 0x13, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x06, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x26, 0x16, 0x17, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x06, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x27, 0x04, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x06, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x27, 0x04, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x06, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x27, 0x13, 0x14, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x06, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x28, 0x04, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06,
    0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x28, 0x04, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06,
    0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x28, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x00, 0x12, 0x03, 0x2b, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x2b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x2b, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b, 0x10,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x17, 0x18, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x2c, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x2c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x2c, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x2c, 0x18, 0x19,
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
