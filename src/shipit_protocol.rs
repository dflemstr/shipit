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
    disconnect(Disconnect),
    update(Update),
    scan(Scan),
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

    // optional .Disconnect disconnect = 4;

    pub fn clear_disconnect(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_disconnect(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(Request_oneof_msg::disconnect(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_disconnect(&mut self, v: Disconnect) {
        self.msg = ::std::option::Option::Some(Request_oneof_msg::disconnect(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_disconnect<'a>(&'a mut self) -> &'a mut Disconnect {
        if let ::std::option::Option::Some(Request_oneof_msg::disconnect(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(Request_oneof_msg::disconnect(Disconnect::new()));
        }
        match self.msg {
            ::std::option::Option::Some(Request_oneof_msg::disconnect(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_disconnect(&mut self) -> Disconnect {
        if self.has_disconnect() {
            match self.msg.take() {
                ::std::option::Option::Some(Request_oneof_msg::disconnect(v)) => v,
                _ => panic!(),
            }
        } else {
            Disconnect::new()
        }
    }

    pub fn get_disconnect<'a>(&'a self) -> &'a Disconnect {
        match self.msg {
            ::std::option::Option::Some(Request_oneof_msg::disconnect(ref v)) => v,
            _ => Disconnect::default_instance(),
        }
    }

    // optional .Update update = 5;

    pub fn clear_update(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_update(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(Request_oneof_msg::update(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_update(&mut self, v: Update) {
        self.msg = ::std::option::Option::Some(Request_oneof_msg::update(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update<'a>(&'a mut self) -> &'a mut Update {
        if let ::std::option::Option::Some(Request_oneof_msg::update(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(Request_oneof_msg::update(Update::new()));
        }
        match self.msg {
            ::std::option::Option::Some(Request_oneof_msg::update(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_update(&mut self) -> Update {
        if self.has_update() {
            match self.msg.take() {
                ::std::option::Option::Some(Request_oneof_msg::update(v)) => v,
                _ => panic!(),
            }
        } else {
            Update::new()
        }
    }

    pub fn get_update<'a>(&'a self) -> &'a Update {
        match self.msg {
            ::std::option::Option::Some(Request_oneof_msg::update(ref v)) => v,
            _ => Update::default_instance(),
        }
    }

    // optional .Scan scan = 6;

    pub fn clear_scan(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_scan(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(Request_oneof_msg::scan(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_scan(&mut self, v: Scan) {
        self.msg = ::std::option::Option::Some(Request_oneof_msg::scan(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_scan<'a>(&'a mut self) -> &'a mut Scan {
        if let ::std::option::Option::Some(Request_oneof_msg::scan(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(Request_oneof_msg::scan(Scan::new()));
        }
        match self.msg {
            ::std::option::Option::Some(Request_oneof_msg::scan(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_scan(&mut self) -> Scan {
        if self.has_scan() {
            match self.msg.take() {
                ::std::option::Option::Some(Request_oneof_msg::scan(v)) => v,
                _ => panic!(),
            }
        } else {
            Scan::new()
        }
    }

    pub fn get_scan<'a>(&'a self) -> &'a Scan {
        match self.msg {
            ::std::option::Option::Some(Request_oneof_msg::scan(ref v)) => v,
            _ => Scan::default_instance(),
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
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.msg = ::std::option::Option::Some(Request_oneof_msg::disconnect(try!(is.read_message())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.msg = ::std::option::Option::Some(Request_oneof_msg::update(try!(is.read_message())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.msg = ::std::option::Option::Some(Request_oneof_msg::scan(try!(is.read_message())));
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
                &Request_oneof_msg::disconnect(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_msg::update(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_msg::scan(ref v) => {
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
                &Request_oneof_msg::disconnect(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Request_oneof_msg::update(ref v) => {
                    try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Request_oneof_msg::scan(ref v) => {
                    try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "disconnect",
                    Request::has_disconnect,
                    Request::get_disconnect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "update",
                    Request::has_update,
                    Request::get_update,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "scan",
                    Request::has_scan,
                    Request::get_scan,
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
        self.clear_disconnect();
        self.clear_update();
        self.clear_scan();
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
    // message fields
    payload: ::protobuf::SingularField<::std::vec::Vec<u8>>,
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
                    payload: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes payload = 1;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.payload.is_none() {
            self.payload.set_default();
        };
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        self.payload.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_payload<'a>(&'a self) -> &'a [u8] {
        match self.payload.as_ref() {
            Some(v) => &v,
            None => &[],
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
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.payload.set_default();
                    try!(is.read_bytes_into(tmp))
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
        for value in self.payload.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.payload.as_ref() {
            try!(os.write_bytes(1, &v));
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
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "payload",
                    Ping::has_payload,
                    Ping::get_payload,
                ));
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
        self.clear_payload();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Ping {
    fn eq(&self, other: &Ping) -> bool {
        self.payload == other.payload &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Ping {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Disconnect {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Disconnect {
    pub fn new() -> Disconnect {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Disconnect {
        static mut instance: ::protobuf::lazy::Lazy<Disconnect> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Disconnect,
        };
        unsafe {
            instance.get(|| {
                Disconnect {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for Disconnect {
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
        ::std::any::TypeId::of::<Disconnect>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Disconnect {
    fn new() -> Disconnect {
        Disconnect::new()
    }

    fn descriptor_static(_: ::std::option::Option<Disconnect>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Disconnect>(
                    "Disconnect",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Disconnect {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Disconnect {
    fn eq(&self, other: &Disconnect) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Disconnect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Update {
    // message fields
    angular_velocity: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Update {
    pub fn new() -> Update {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Update {
        static mut instance: ::protobuf::lazy::Lazy<Update> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Update,
        };
        unsafe {
            instance.get(|| {
                Update {
                    angular_velocity: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional double angular_velocity = 1;

    pub fn clear_angular_velocity(&mut self) {
        self.angular_velocity = ::std::option::Option::None;
    }

    pub fn has_angular_velocity(&self) -> bool {
        self.angular_velocity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angular_velocity(&mut self, v: f64) {
        self.angular_velocity = ::std::option::Option::Some(v);
    }

    pub fn get_angular_velocity<'a>(&self) -> f64 {
        self.angular_velocity.unwrap_or(0.)
    }
}

impl ::protobuf::Message for Update {
    fn is_initialized(&self) -> bool {
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
                    self.angular_velocity = ::std::option::Option::Some(tmp);
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
        if self.angular_velocity.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.angular_velocity {
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
        ::std::any::TypeId::of::<Update>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Update {
    fn new() -> Update {
        Update::new()
    }

    fn descriptor_static(_: ::std::option::Option<Update>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "angular_velocity",
                    Update::has_angular_velocity,
                    Update::get_angular_velocity,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Update>(
                    "Update",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Update {
    fn clear(&mut self) {
        self.clear_angular_velocity();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Update {
    fn eq(&self, other: &Update) -> bool {
        self.angular_velocity == other.angular_velocity &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Update {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Scan {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Scan {
    pub fn new() -> Scan {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Scan {
        static mut instance: ::protobuf::lazy::Lazy<Scan> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Scan,
        };
        unsafe {
            instance.get(|| {
                Scan {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for Scan {
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
        ::std::any::TypeId::of::<Scan>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Scan {
    fn new() -> Scan {
        Scan::new()
    }

    fn descriptor_static(_: ::std::option::Option<Scan>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Scan>(
                    "Scan",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Scan {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Scan {
    fn eq(&self, other: &Scan) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Scan {
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
    disconnected(Disconnected),
    updated(Updated),
    scanned(Scanned),
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

    // optional .Disconnected disconnected = 4;

    pub fn clear_disconnected(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_disconnected(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(Response_oneof_msg::disconnected(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_disconnected(&mut self, v: Disconnected) {
        self.msg = ::std::option::Option::Some(Response_oneof_msg::disconnected(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_disconnected<'a>(&'a mut self) -> &'a mut Disconnected {
        if let ::std::option::Option::Some(Response_oneof_msg::disconnected(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(Response_oneof_msg::disconnected(Disconnected::new()));
        }
        match self.msg {
            ::std::option::Option::Some(Response_oneof_msg::disconnected(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_disconnected(&mut self) -> Disconnected {
        if self.has_disconnected() {
            match self.msg.take() {
                ::std::option::Option::Some(Response_oneof_msg::disconnected(v)) => v,
                _ => panic!(),
            }
        } else {
            Disconnected::new()
        }
    }

    pub fn get_disconnected<'a>(&'a self) -> &'a Disconnected {
        match self.msg {
            ::std::option::Option::Some(Response_oneof_msg::disconnected(ref v)) => v,
            _ => Disconnected::default_instance(),
        }
    }

    // optional .Updated updated = 5;

    pub fn clear_updated(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_updated(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(Response_oneof_msg::updated(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_updated(&mut self, v: Updated) {
        self.msg = ::std::option::Option::Some(Response_oneof_msg::updated(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_updated<'a>(&'a mut self) -> &'a mut Updated {
        if let ::std::option::Option::Some(Response_oneof_msg::updated(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(Response_oneof_msg::updated(Updated::new()));
        }
        match self.msg {
            ::std::option::Option::Some(Response_oneof_msg::updated(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_updated(&mut self) -> Updated {
        if self.has_updated() {
            match self.msg.take() {
                ::std::option::Option::Some(Response_oneof_msg::updated(v)) => v,
                _ => panic!(),
            }
        } else {
            Updated::new()
        }
    }

    pub fn get_updated<'a>(&'a self) -> &'a Updated {
        match self.msg {
            ::std::option::Option::Some(Response_oneof_msg::updated(ref v)) => v,
            _ => Updated::default_instance(),
        }
    }

    // optional .Scanned scanned = 6;

    pub fn clear_scanned(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_scanned(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(Response_oneof_msg::scanned(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_scanned(&mut self, v: Scanned) {
        self.msg = ::std::option::Option::Some(Response_oneof_msg::scanned(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_scanned<'a>(&'a mut self) -> &'a mut Scanned {
        if let ::std::option::Option::Some(Response_oneof_msg::scanned(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(Response_oneof_msg::scanned(Scanned::new()));
        }
        match self.msg {
            ::std::option::Option::Some(Response_oneof_msg::scanned(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_scanned(&mut self) -> Scanned {
        if self.has_scanned() {
            match self.msg.take() {
                ::std::option::Option::Some(Response_oneof_msg::scanned(v)) => v,
                _ => panic!(),
            }
        } else {
            Scanned::new()
        }
    }

    pub fn get_scanned<'a>(&'a self) -> &'a Scanned {
        match self.msg {
            ::std::option::Option::Some(Response_oneof_msg::scanned(ref v)) => v,
            _ => Scanned::default_instance(),
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
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.msg = ::std::option::Option::Some(Response_oneof_msg::disconnected(try!(is.read_message())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.msg = ::std::option::Option::Some(Response_oneof_msg::updated(try!(is.read_message())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.msg = ::std::option::Option::Some(Response_oneof_msg::scanned(try!(is.read_message())));
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
                &Response_oneof_msg::disconnected(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_msg::updated(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_msg::scanned(ref v) => {
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
                &Response_oneof_msg::disconnected(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Response_oneof_msg::updated(ref v) => {
                    try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Response_oneof_msg::scanned(ref v) => {
                    try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "disconnected",
                    Response::has_disconnected,
                    Response::get_disconnected,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "updated",
                    Response::has_updated,
                    Response::get_updated,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "scanned",
                    Response::has_scanned,
                    Response::get_scanned,
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
        self.clear_disconnected();
        self.clear_updated();
        self.clear_scanned();
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
    // message fields
    payload: ::protobuf::SingularField<::std::vec::Vec<u8>>,
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
                    payload: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes payload = 1;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.payload.is_none() {
            self.payload.set_default();
        };
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        self.payload.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_payload<'a>(&'a self) -> &'a [u8] {
        match self.payload.as_ref() {
            Some(v) => &v,
            None => &[],
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
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.payload.set_default();
                    try!(is.read_bytes_into(tmp))
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
        for value in self.payload.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.payload.as_ref() {
            try!(os.write_bytes(1, &v));
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
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "payload",
                    Pong::has_payload,
                    Pong::get_payload,
                ));
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
        self.clear_payload();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Pong {
    fn eq(&self, other: &Pong) -> bool {
        self.payload == other.payload &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Pong {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Disconnected {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Disconnected {
    pub fn new() -> Disconnected {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Disconnected {
        static mut instance: ::protobuf::lazy::Lazy<Disconnected> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Disconnected,
        };
        unsafe {
            instance.get(|| {
                Disconnected {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for Disconnected {
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
        ::std::any::TypeId::of::<Disconnected>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Disconnected {
    fn new() -> Disconnected {
        Disconnected::new()
    }

    fn descriptor_static(_: ::std::option::Option<Disconnected>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Disconnected>(
                    "Disconnected",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Disconnected {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Disconnected {
    fn eq(&self, other: &Disconnected) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Disconnected {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Updated {
    // message fields
    angular_velocity: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Updated {
    pub fn new() -> Updated {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Updated {
        static mut instance: ::protobuf::lazy::Lazy<Updated> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Updated,
        };
        unsafe {
            instance.get(|| {
                Updated {
                    angular_velocity: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required double angular_velocity = 1;

    pub fn clear_angular_velocity(&mut self) {
        self.angular_velocity = ::std::option::Option::None;
    }

    pub fn has_angular_velocity(&self) -> bool {
        self.angular_velocity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angular_velocity(&mut self, v: f64) {
        self.angular_velocity = ::std::option::Option::Some(v);
    }

    pub fn get_angular_velocity<'a>(&self) -> f64 {
        self.angular_velocity.unwrap_or(0.)
    }
}

impl ::protobuf::Message for Updated {
    fn is_initialized(&self) -> bool {
        if self.angular_velocity.is_none() {
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
                    self.angular_velocity = ::std::option::Option::Some(tmp);
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
        if self.angular_velocity.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.angular_velocity {
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
        ::std::any::TypeId::of::<Updated>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Updated {
    fn new() -> Updated {
        Updated::new()
    }

    fn descriptor_static(_: ::std::option::Option<Updated>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "angular_velocity",
                    Updated::has_angular_velocity,
                    Updated::get_angular_velocity,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Updated>(
                    "Updated",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Updated {
    fn clear(&mut self) {
        self.clear_angular_velocity();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Updated {
    fn eq(&self, other: &Updated) -> bool {
        self.angular_velocity == other.angular_velocity &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Updated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Scanned {
    // message fields
    hit: ::protobuf::RepeatedField<Scanned_Hit>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Scanned {
    pub fn new() -> Scanned {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Scanned {
        static mut instance: ::protobuf::lazy::Lazy<Scanned> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Scanned,
        };
        unsafe {
            instance.get(|| {
                Scanned {
                    hit: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .Scanned.Hit hit = 1;

    pub fn clear_hit(&mut self) {
        self.hit.clear();
    }

    // Param is passed by value, moved
    pub fn set_hit(&mut self, v: ::protobuf::RepeatedField<Scanned_Hit>) {
        self.hit = v;
    }

    // Mutable pointer to the field.
    pub fn mut_hit<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Scanned_Hit> {
        &mut self.hit
    }

    // Take field
    pub fn take_hit(&mut self) -> ::protobuf::RepeatedField<Scanned_Hit> {
        ::std::mem::replace(&mut self.hit, ::protobuf::RepeatedField::new())
    }

    pub fn get_hit<'a>(&'a self) -> &'a [Scanned_Hit] {
        &self.hit
    }
}

impl ::protobuf::Message for Scanned {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.hit));
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
        for value in self.hit.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.hit.iter() {
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
        ::std::any::TypeId::of::<Scanned>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Scanned {
    fn new() -> Scanned {
        Scanned::new()
    }

    fn descriptor_static(_: ::std::option::Option<Scanned>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "hit",
                    Scanned::get_hit,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Scanned>(
                    "Scanned",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Scanned {
    fn clear(&mut self) {
        self.clear_hit();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Scanned {
    fn eq(&self, other: &Scanned) -> bool {
        self.hit == other.hit &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Scanned {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Scanned_Hit {
    // message fields
    distance: ::std::option::Option<f64>,
    angle: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Scanned_Hit {
    pub fn new() -> Scanned_Hit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Scanned_Hit {
        static mut instance: ::protobuf::lazy::Lazy<Scanned_Hit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Scanned_Hit,
        };
        unsafe {
            instance.get(|| {
                Scanned_Hit {
                    distance: ::std::option::Option::None,
                    angle: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required double distance = 1;

    pub fn clear_distance(&mut self) {
        self.distance = ::std::option::Option::None;
    }

    pub fn has_distance(&self) -> bool {
        self.distance.is_some()
    }

    // Param is passed by value, moved
    pub fn set_distance(&mut self, v: f64) {
        self.distance = ::std::option::Option::Some(v);
    }

    pub fn get_distance<'a>(&self) -> f64 {
        self.distance.unwrap_or(0.)
    }

    // required double angle = 2;

    pub fn clear_angle(&mut self) {
        self.angle = ::std::option::Option::None;
    }

    pub fn has_angle(&self) -> bool {
        self.angle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angle(&mut self, v: f64) {
        self.angle = ::std::option::Option::Some(v);
    }

    pub fn get_angle<'a>(&self) -> f64 {
        self.angle.unwrap_or(0.)
    }
}

impl ::protobuf::Message for Scanned_Hit {
    fn is_initialized(&self) -> bool {
        if self.distance.is_none() {
            return false;
        };
        if self.angle.is_none() {
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
                    self.distance = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.angle = ::std::option::Option::Some(tmp);
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
        if self.distance.is_some() {
            my_size += 9;
        };
        if self.angle.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.distance {
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.angle {
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
        ::std::any::TypeId::of::<Scanned_Hit>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Scanned_Hit {
    fn new() -> Scanned_Hit {
        Scanned_Hit::new()
    }

    fn descriptor_static(_: ::std::option::Option<Scanned_Hit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "distance",
                    Scanned_Hit::has_distance,
                    Scanned_Hit::get_distance,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "angle",
                    Scanned_Hit::has_angle,
                    Scanned_Hit::get_angle,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Scanned_Hit>(
                    "Scanned_Hit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Scanned_Hit {
    fn clear(&mut self) {
        self.clear_distance();
        self.clear_angle();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Scanned_Hit {
    fn eq(&self, other: &Scanned_Hit) -> bool {
        self.distance == other.distance &&
        self.angle == other.angle &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Scanned_Hit {
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
    0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xca, 0x01, 0x0a, 0x07,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x17, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x48, 0x00,
    0x12, 0x1d, 0x0a, 0x08, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x79, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x09, 0x2e, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x79, 0x48, 0x00, 0x12,
    0x15, 0x0a, 0x04, 0x70, 0x69, 0x6e, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x05, 0x2e,
    0x50, 0x69, 0x6e, 0x67, 0x48, 0x00, 0x12, 0x21, 0x0a, 0x0a, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x6e,
    0x6e, 0x65, 0x63, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x44, 0x69, 0x73,
    0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x48, 0x00, 0x12, 0x19, 0x0a, 0x06, 0x75, 0x70, 0x64,
    0x61, 0x74, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x55, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x48, 0x00, 0x12, 0x15, 0x0a, 0x04, 0x73, 0x63, 0x61, 0x6e, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x05, 0x2e, 0x53, 0x63, 0x61, 0x6e, 0x48, 0x00, 0x12, 0x14, 0x0a, 0x0c, 0x61,
    0x63, 0x63, 0x65, 0x73, 0x73, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x40, 0x20, 0x01, 0x28,
    0x09, 0x42, 0x05, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x22, 0x18, 0x0a, 0x08, 0x49, 0x64, 0x65, 0x6e,
    0x74, 0x69, 0x66, 0x79, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x22, 0x17, 0x0a, 0x04, 0x50, 0x69, 0x6e, 0x67, 0x12, 0x0f, 0x0a, 0x07, 0x70, 0x61,
    0x79, 0x6c, 0x6f, 0x61, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x0c, 0x0a, 0x0a, 0x44,
    0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x22, 0x22, 0x0a, 0x06, 0x55, 0x70, 0x64,
    0x61, 0x74, 0x65, 0x12, 0x18, 0x0a, 0x10, 0x61, 0x6e, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x5f, 0x76,
    0x65, 0x6c, 0x6f, 0x63, 0x69, 0x74, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x01, 0x22, 0x06, 0x0a,
    0x04, 0x53, 0x63, 0x61, 0x6e, 0x22, 0xc5, 0x01, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x17, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x06, 0x2e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x48, 0x00, 0x12, 0x21, 0x0a, 0x0a, 0x69,
    0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x0b, 0x2e, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x64, 0x48, 0x00, 0x12, 0x15,
    0x0a, 0x04, 0x70, 0x6f, 0x6e, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x50,
    0x6f, 0x6e, 0x67, 0x48, 0x00, 0x12, 0x25, 0x0a, 0x0c, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e,
    0x65, 0x63, 0x74, 0x65, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x44, 0x69,
    0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x65, 0x64, 0x48, 0x00, 0x12, 0x1b, 0x0a, 0x07,
    0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x08, 0x2e,
    0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x48, 0x00, 0x12, 0x1b, 0x0a, 0x07, 0x73, 0x63, 0x61,
    0x6e, 0x6e, 0x65, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x53, 0x63, 0x61,
    0x6e, 0x6e, 0x65, 0x64, 0x48, 0x00, 0x42, 0x05, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x22, 0x37, 0x0a,
    0x0a, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x64, 0x12, 0x14, 0x0a, 0x0c, 0x61,
    0x63, 0x63, 0x65, 0x73, 0x73, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x09, 0x12, 0x13, 0x0a, 0x0b, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x69, 0x6e, 0x66, 0x6f,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x22, 0x17, 0x0a, 0x04, 0x50, 0x6f, 0x6e, 0x67, 0x12, 0x0f,
    0x0a, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x22,
    0x0e, 0x0a, 0x0c, 0x44, 0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x65, 0x64, 0x22,
    0x23, 0x0a, 0x07, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x12, 0x18, 0x0a, 0x10, 0x61, 0x6e,
    0x67, 0x75, 0x6c, 0x61, 0x72, 0x5f, 0x76, 0x65, 0x6c, 0x6f, 0x63, 0x69, 0x74, 0x79, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x01, 0x22, 0x4c, 0x0a, 0x07, 0x53, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x64, 0x12,
    0x19, 0x0a, 0x03, 0x68, 0x69, 0x74, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x53,
    0x63, 0x61, 0x6e, 0x6e, 0x65, 0x64, 0x2e, 0x48, 0x69, 0x74, 0x1a, 0x26, 0x0a, 0x03, 0x48, 0x69,
    0x74, 0x12, 0x10, 0x0a, 0x08, 0x64, 0x69, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x01, 0x12, 0x0d, 0x0a, 0x05, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x01, 0x22, 0x93, 0x01, 0x0a, 0x05, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x19, 0x0a, 0x04,
    0x6b, 0x69, 0x6e, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x0b, 0x2e, 0x45, 0x72, 0x72,
    0x6f, 0x72, 0x2e, 0x4b, 0x69, 0x6e, 0x64, 0x12, 0x0b, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x09, 0x22, 0x62, 0x0a, 0x04, 0x4b, 0x69, 0x6e, 0x64, 0x12, 0x0e, 0x0a, 0x0a,
    0x57, 0x49, 0x52, 0x45, 0x5f, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x10, 0x01, 0x12, 0x0c, 0x0a, 0x08,
    0x49, 0x4f, 0x5f, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x10, 0x02, 0x12, 0x13, 0x0a, 0x0f, 0x55, 0x4e,
    0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x5f, 0x52, 0x45, 0x51, 0x55, 0x45, 0x53, 0x54, 0x10, 0x03, 0x12,
    0x10, 0x0a, 0x0c, 0x55, 0x4e, 0x41, 0x55, 0x54, 0x48, 0x4f, 0x52, 0x49, 0x5a, 0x45, 0x44, 0x10,
    0x04, 0x12, 0x15, 0x0a, 0x11, 0x50, 0x4c, 0x41, 0x59, 0x45, 0x52, 0x5f, 0x4e, 0x41, 0x4d, 0x45,
    0x5f, 0x54, 0x41, 0x4b, 0x45, 0x4e, 0x10, 0x05, 0x4a, 0xdd, 0x11, 0x0a, 0x06, 0x12, 0x04, 0x01,
    0x00, 0x4f, 0x01, 0x0a, 0x16, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x01, 0x00, 0x0b, 0x01, 0x1a,
    0x0a, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x01, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x08, 0x00, 0x12,
    0x04, 0x02, 0x02, 0x09, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x08, 0x00, 0x01, 0x12, 0x03,
    0x02, 0x08, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x04, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x03, 0x04, 0x09, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x0a, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x03, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x04, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x04, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x04, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x04, 0x18,
    0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x05, 0x04, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x05, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x05, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x05, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x06, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03,
    0x06, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x06, 0x0f,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x06, 0x1c, 0x1d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x07, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x07, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x07, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x07, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03,
    0x08, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x06, 0x12, 0x03, 0x08, 0x04,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x08, 0x09, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x08, 0x10, 0x11, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x0a, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x06, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06,
    0x05, 0x12, 0x03, 0x0a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x0a, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x0a,
    0x21, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0d, 0x00, 0x0f, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x00, 0x12, 0x03, 0x0e, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x12,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x19, 0x1a, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x11, 0x00, 0x13, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x11, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12,
    0x03, 0x12, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x12,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x12, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x11, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x12, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x03, 0x12, 0x04, 0x15, 0x00, 0x16, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12,
    0x03, 0x15, 0x08, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x18, 0x00, 0x1a, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x18, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x19, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x19, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x19, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x19, 0x12, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x25,
    0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x1c, 0x00, 0x1d, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x1c, 0x08, 0x0c, 0x0a, 0x17, 0x0a, 0x02, 0x04, 0x06, 0x12,
    0x04, 0x20, 0x00, 0x29, 0x01, 0x1a, 0x0b, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x20, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x06, 0x08, 0x00, 0x12, 0x04, 0x21, 0x02, 0x28, 0x03, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x08, 0x00, 0x01, 0x12, 0x03, 0x21, 0x08, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x00, 0x12, 0x03, 0x22, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x22, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x22, 0x0a, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x22, 0x12,
    0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x23, 0x04, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x06, 0x12, 0x03, 0x23, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x23, 0x0f, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x23, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02,
    0x12, 0x03, 0x24, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x06, 0x12, 0x03,
    0x24, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x24, 0x09,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x24, 0x10, 0x11, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x25, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x03, 0x06, 0x12, 0x03, 0x25, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x25, 0x11, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x25, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x04, 0x12, 0x03,
    0x26, 0x04, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x06, 0x12, 0x03, 0x26, 0x04,
    0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x01, 0x12, 0x03, 0x26, 0x0c, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x03, 0x12, 0x03, 0x26, 0x16, 0x17, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x05, 0x12, 0x03, 0x27, 0x04, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x05, 0x06, 0x12, 0x03, 0x27, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x27, 0x0c, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x03, 0x12,
    0x03, 0x27, 0x16, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x2b, 0x00, 0x2e, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x2b, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x2c, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x2c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x2c, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2c, 0x21,
    0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x02, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x2d, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x2d, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x30, 0x00,
    0x32, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x30, 0x08, 0x0c, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x31, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x31, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x31, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x31, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x31, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x34, 0x00, 0x35, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x34, 0x08, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x0a, 0x12, 0x04, 0x37, 0x00, 0x39, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03,
    0x37, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x38, 0x02, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x38, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x03, 0x38, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x38, 0x12, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x38, 0x25, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04,
    0x3b, 0x00, 0x42, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x3b, 0x08, 0x0f,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x03, 0x00, 0x12, 0x04, 0x3c, 0x02, 0x3f, 0x03, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x03, 0x00, 0x01, 0x12, 0x03, 0x3c, 0x0a, 0x0d, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x0b, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x3d, 0x04, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x0b, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3d, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x0b, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3d, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x0b, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3d, 0x14, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x0b, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3d, 0x1f, 0x20, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x0b, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x3e, 0x04, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b,
    0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x3e, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b,
    0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x3e, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b,
    0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3e, 0x14, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b,
    0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3e, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b,
    0x02, 0x00, 0x12, 0x03, 0x41, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x41, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x41, 0x0b, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x41, 0x0f,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x41, 0x15, 0x16, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x44, 0x00, 0x4f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x0c, 0x01, 0x12, 0x03, 0x44, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x04, 0x00, 0x12,
    0x04, 0x45, 0x02, 0x4b, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x45, 0x07, 0x0b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x46,
    0x04, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x46,
    0x04, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x46,
    0x11, 0x12, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x47, 0x04,
    0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x47, 0x04,
    0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x47, 0x0f,
    0x10, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x48, 0x04, 0x18,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x48, 0x04, 0x13,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x48, 0x16, 0x17,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x49, 0x04, 0x15, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x49, 0x04, 0x10, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x49, 0x13, 0x14, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x4a, 0x04, 0x1a, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x4a, 0x04, 0x15, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x4a, 0x18, 0x19, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x4d, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x4d, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x4d, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x4d, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x03, 0x4e, 0x02, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x03, 0x4e, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x05, 0x12, 0x03, 0x4e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4e, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x4e, 0x18, 0x19,
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
