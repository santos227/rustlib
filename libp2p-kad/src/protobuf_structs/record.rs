// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Record {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    author: ::protobuf::SingularField<::std::string::String>,
    signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    timeReceived: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Record {}

impl Record {
    pub fn new() -> Record {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Record {
        static mut instance: ::protobuf::lazy::Lazy<Record> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Record,
        };
        unsafe {
            instance.get(Record::new)
        }
    }

    // optional string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        if self.key.is_none() {
            self.key.set_default();
        }
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        self.key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        match self.key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.key
    }

    // optional bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.value
    }

    // optional string author = 3;

    pub fn clear_author(&mut self) {
        self.author.clear();
    }

    pub fn has_author(&self) -> bool {
        self.author.is_some()
    }

    // Param is passed by value, moved
    pub fn set_author(&mut self, v: ::std::string::String) {
        self.author = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_author(&mut self) -> &mut ::std::string::String {
        if self.author.is_none() {
            self.author.set_default();
        }
        self.author.as_mut().unwrap()
    }

    // Take field
    pub fn take_author(&mut self) -> ::std::string::String {
        self.author.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_author(&self) -> &str {
        match self.author.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_author_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.author
    }

    fn mut_author_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.author
    }

    // optional bytes signature = 4;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.signature.is_none() {
            self.signature.set_default();
        }
        self.signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        self.signature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_signature(&self) -> &[u8] {
        match self.signature.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_signature_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.signature
    }

    // optional string timeReceived = 5;

    pub fn clear_timeReceived(&mut self) {
        self.timeReceived.clear();
    }

    pub fn has_timeReceived(&self) -> bool {
        self.timeReceived.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timeReceived(&mut self, v: ::std::string::String) {
        self.timeReceived = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_timeReceived(&mut self) -> &mut ::std::string::String {
        if self.timeReceived.is_none() {
            self.timeReceived.set_default();
        }
        self.timeReceived.as_mut().unwrap()
    }

    // Take field
    pub fn take_timeReceived(&mut self) -> ::std::string::String {
        self.timeReceived.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_timeReceived(&self) -> &str {
        match self.timeReceived.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_timeReceived_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.timeReceived
    }

    fn mut_timeReceived_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.timeReceived
    }
}

impl ::protobuf::Message for Record {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.author)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.signature)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.timeReceived)?;
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
        if let Some(ref v) = self.key.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(ref v) = self.author.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.signature.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        if let Some(ref v) = self.timeReceived.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.key.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.value.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(ref v) = self.author.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.signature.as_ref() {
            os.write_bytes(4, &v)?;
        }
        if let Some(ref v) = self.timeReceived.as_ref() {
            os.write_string(5, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Record {
    fn new() -> Record {
        Record::new()
    }

    fn descriptor_static(_: ::std::option::Option<Record>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    Record::get_key_for_reflect,
                    Record::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    Record::get_value_for_reflect,
                    Record::mut_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "author",
                    Record::get_author_for_reflect,
                    Record::mut_author_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    Record::get_signature_for_reflect,
                    Record::mut_signature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "timeReceived",
                    Record::get_timeReceived_for_reflect,
                    Record::mut_timeReceived_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Record>(
                    "Record",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Record {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.clear_author();
        self.clear_signature();
        self.clear_timeReceived();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Record {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Record {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0crecord.proto\x12\trecord.pb\"\x8a\x01\n\x06Record\x12\x10\n\x03key\
    \x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\x0cR\x05\
    value\x12\x16\n\x06author\x18\x03\x20\x01(\tR\x06author\x12\x1c\n\tsigna\
    ture\x18\x04\x20\x01(\x0cR\tsignature\x12\"\n\x0ctimeReceived\x18\x05\
    \x20\x01(\tR\x0ctimeReceivedJ\xac\x05\n\x06\x12\x04\0\0\x14\x01\n\x08\n\
    \x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\x08\x11\nX\n\x02\
    \x04\0\x12\x04\x05\0\x14\x01\x1aL\x20Record\x20represents\x20a\x20dht\
    \x20record\x20that\x20contains\x20a\x20value\n\x20for\x20a\x20key\x20val\
    ue\x20pair\n\n\n\n\x03\x04\0\x01\x12\x03\x05\x08\x0e\n2\n\x04\x04\0\x02\
    \0\x12\x03\x07\x08\x20\x1a%\x20The\x20key\x20that\x20references\x20this\
    \x20record\n\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x07\x08\x10\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03\x07\x11\x17\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x07\x18\x1b\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x07\x1e\x1f\n6\n\x04\
    \x04\0\x02\x01\x12\x03\n\x08!\x1a)\x20The\x20actual\x20value\x20this\x20\
    record\x20is\x20storing\n\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\n\x08\
    \x10\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\n\x11\x16\n\x0c\n\x05\x04\0\
    \x02\x01\x01\x12\x03\n\x17\x1c\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\n\
    \x1f\x20\n-\n\x04\x04\0\x02\x02\x12\x03\r\x08#\x1a\x20\x20hash\x20of\x20\
    the\x20authors\x20public\x20key\n\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03\
    \r\x08\x10\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\r\x11\x17\n\x0c\n\x05\
    \x04\0\x02\x02\x01\x12\x03\r\x18\x1e\n\x0c\n\x05\x04\0\x02\x02\x03\x12\
    \x03\r!\"\n7\n\x04\x04\0\x02\x03\x12\x03\x10\x08%\x1a*\x20A\x20PKI\x20si\
    gnature\x20for\x20the\x20key+value+author\n\n\x0c\n\x05\x04\0\x02\x03\
    \x04\x12\x03\x10\x08\x10\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x10\x11\
    \x16\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x10\x17\x20\n\x0c\n\x05\x04\0\
    \x02\x03\x03\x12\x03\x10#$\n<\n\x04\x04\0\x02\x04\x12\x03\x13\x08)\x1a/\
    \x20Time\x20the\x20record\x20was\x20received,\x20set\x20by\x20receiver\n\
    \n\x0c\n\x05\x04\0\x02\x04\x04\x12\x03\x13\x08\x10\n\x0c\n\x05\x04\0\x02\
    \x04\x05\x12\x03\x13\x11\x17\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x13\
    \x18$\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x13'(\
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
