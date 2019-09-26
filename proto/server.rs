// This file is generated by rust-protobuf 2.8.1. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `proto/server.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_1;

#[derive(PartialEq,Clone,Default)]
pub struct EchoRequest {
    // message fields
    pub message: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a EchoRequest {
    fn default() -> &'a EchoRequest {
        <EchoRequest as ::protobuf::Message>::default_instance()
    }
}

impl EchoRequest {
    pub fn new() -> EchoRequest {
        ::std::default::Default::default()
    }

    // string message = 1;


    pub fn get_message(&self) -> &str {
        &self.message
    }
    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.message, ::std::string::String::new())
    }
}

impl ::protobuf::Message for EchoRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.message)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.message);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.message.is_empty() {
            os.write_string(1, &self.message)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> EchoRequest {
        EchoRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    |m: &EchoRequest| { &m.message },
                    |m: &mut EchoRequest| { &mut m.message },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EchoRequest>(
                    "EchoRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static EchoRequest {
        static mut instance: ::protobuf::lazy::Lazy<EchoRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EchoRequest,
        };
        unsafe {
            instance.get(EchoRequest::new)
        }
    }
}

impl ::protobuf::Clear for EchoRequest {
    fn clear(&mut self) {
        self.message.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EchoRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EchoRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EchoResponse {
    // message fields
    pub message: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a EchoResponse {
    fn default() -> &'a EchoResponse {
        <EchoResponse as ::protobuf::Message>::default_instance()
    }
}

impl EchoResponse {
    pub fn new() -> EchoResponse {
        ::std::default::Default::default()
    }

    // string message = 1;


    pub fn get_message(&self) -> &str {
        &self.message
    }
    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.message, ::std::string::String::new())
    }
}

impl ::protobuf::Message for EchoResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.message)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.message);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.message.is_empty() {
            os.write_string(1, &self.message)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> EchoResponse {
        EchoResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    |m: &EchoResponse| { &m.message },
                    |m: &mut EchoResponse| { &mut m.message },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EchoResponse>(
                    "EchoResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static EchoResponse {
        static mut instance: ::protobuf::lazy::Lazy<EchoResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EchoResponse,
        };
        unsafe {
            instance.get(EchoResponse::new)
        }
    }
}

impl ::protobuf::Clear for EchoResponse {
    fn clear(&mut self) {
        self.message.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EchoResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EchoResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12proto/server.proto\x12\x04echo\"'\n\x0bEchoRequest\x12\x18\n\x07me\
    ssage\x18\x01\x20\x01(\tR\x07message\"(\n\x0cEchoResponse\x12\x18\n\x07m\
    essage\x18\x01\x20\x01(\tR\x07message2>\n\x0bEchoService\x12/\n\x04Echo\
    \x12\x11.echo.EchoRequest\x1a\x12.echo.EchoResponse\"\0J\xa7\x02\n\x06\
    \x12\x04\0\0\x0e\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\
    \x03\x02\x08\x0c\n\n\n\x02\x06\0\x12\x04\x04\0\x06\x01\n\n\n\x03\x06\0\
    \x01\x12\x03\x04\x08\x13\n\x0b\n\x04\x06\0\x02\0\x12\x03\x05\x044\n\x0c\
    \n\x05\x06\0\x02\0\x01\x12\x03\x05\x08\x0c\n\x0c\n\x05\x06\0\x02\0\x02\
    \x12\x03\x05\x0e\x19\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x05$0\n\n\n\x02\
    \x04\0\x12\x04\x08\0\n\x01\n\n\n\x03\x04\0\x01\x12\x03\x08\x08\x13\n\x0b\
    \n\x04\x04\0\x02\0\x12\x03\t\x04\x17\n\r\n\x05\x04\0\x02\0\x04\x12\x04\t\
    \x04\x08\x15\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\t\x04\n\n\x0c\n\x05\x04\
    \0\x02\0\x01\x12\x03\t\x0b\x12\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\t\x15\
    \x16\n\n\n\x02\x04\x01\x12\x04\x0c\0\x0e\x01\n\n\n\x03\x04\x01\x01\x12\
    \x03\x0c\x08\x14\n\x0b\n\x04\x04\x01\x02\0\x12\x03\r\x04\x17\n\r\n\x05\
    \x04\x01\x02\0\x04\x12\x04\r\x04\x0c\x16\n\x0c\n\x05\x04\x01\x02\0\x05\
    \x12\x03\r\x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\r\x0b\x12\n\x0c\n\
    \x05\x04\x01\x02\0\x03\x12\x03\r\x15\x16b\x06proto3\
";

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
