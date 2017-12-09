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
pub struct ArtistInfo {
    // message fields
    pub id: i32,
    pub title: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ArtistInfo {}

impl ArtistInfo {
    pub fn new() -> ArtistInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ArtistInfo {
        static mut instance: ::protobuf::lazy::Lazy<ArtistInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ArtistInfo,
        };
        unsafe {
            instance.get(ArtistInfo::new)
        }
    }

    // int32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i32) {
        self.id = v;
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &i32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.id
    }

    // string title = 11;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_for_reflect(&self) -> &::std::string::String {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }
}

impl ::protobuf::Message for ArtistInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.id = tmp;
                },
                11 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
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
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.title);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_int32(1, self.id)?;
        }
        if !self.title.is_empty() {
            os.write_string(11, &self.title)?;
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

impl ::protobuf::MessageStatic for ArtistInfo {
    fn new() -> ArtistInfo {
        ArtistInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<ArtistInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "id",
                    ArtistInfo::get_id_for_reflect,
                    ArtistInfo::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    ArtistInfo::get_title_for_reflect,
                    ArtistInfo::mut_title_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ArtistInfo>(
                    "ArtistInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ArtistInfo {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_title();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ArtistInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ArtistInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ArtistList {
    // message fields
    pub artists: ::protobuf::RepeatedField<ArtistInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ArtistList {}

impl ArtistList {
    pub fn new() -> ArtistList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ArtistList {
        static mut instance: ::protobuf::lazy::Lazy<ArtistList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ArtistList,
        };
        unsafe {
            instance.get(ArtistList::new)
        }
    }

    // repeated .uptempo.ArtistInfo artists = 1;

    pub fn clear_artists(&mut self) {
        self.artists.clear();
    }

    // Param is passed by value, moved
    pub fn set_artists(&mut self, v: ::protobuf::RepeatedField<ArtistInfo>) {
        self.artists = v;
    }

    // Mutable pointer to the field.
    pub fn mut_artists(&mut self) -> &mut ::protobuf::RepeatedField<ArtistInfo> {
        &mut self.artists
    }

    // Take field
    pub fn take_artists(&mut self) -> ::protobuf::RepeatedField<ArtistInfo> {
        ::std::mem::replace(&mut self.artists, ::protobuf::RepeatedField::new())
    }

    pub fn get_artists(&self) -> &[ArtistInfo] {
        &self.artists
    }

    fn get_artists_for_reflect(&self) -> &::protobuf::RepeatedField<ArtistInfo> {
        &self.artists
    }

    fn mut_artists_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ArtistInfo> {
        &mut self.artists
    }
}

impl ::protobuf::Message for ArtistList {
    fn is_initialized(&self) -> bool {
        for v in &self.artists {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.artists)?;
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
        for value in &self.artists {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.artists {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for ArtistList {
    fn new() -> ArtistList {
        ArtistList::new()
    }

    fn descriptor_static(_: ::std::option::Option<ArtistList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ArtistInfo>>(
                    "artists",
                    ArtistList::get_artists_for_reflect,
                    ArtistList::mut_artists_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ArtistList>(
                    "ArtistList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ArtistList {
    fn clear(&mut self) {
        self.clear_artists();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ArtistList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ArtistList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cartist.proto\x12\x07uptempo\">\n\nArtistInfo\x12\x0e\n\x02id\x18\
    \x01\x20\x01(\x05R\x02id\x12\x14\n\x05title\x18\x0b\x20\x01(\tR\x05title\
    J\x04\x08\x02\x10\x0bJ\x04\x08\x0c\x10\x1f\";\n\nArtistList\x12-\n\x07ar\
    tists\x18\x01\x20\x03(\x0b2\x13.uptempo.ArtistInfoR\x07artistsJ\xb7\x03\
    \n\x06\x12\x04\0\0\x10\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\
    \x02\x12\x03\x02\x08\x0f\n\n\n\x02\x04\0\x12\x04\x04\0\x0c\x01\n\n\n\x03\
    \x04\0\x01\x12\x03\x04\x08\x12\n#\n\x03\x04\0\t\x12\x03\x06\x0b\x13\x1a\
    \x17\x20System-Level\x20Metadata\n\n\x0b\n\x04\x04\0\t\0\x12\x03\x06\x0b\
    \x12\n\x0c\n\x05\x04\0\t\0\x01\x12\x03\x06\x0b\x0c\n\x0c\n\x05\x04\0\t\0\
    \x02\x12\x03\x06\x10\x12\n\x0b\n\x04\x04\0\x02\0\x12\x03\x07\x02\x0f\n\r\
    \n\x05\x04\0\x02\0\x04\x12\x04\x07\x02\x06\x13\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03\x07\x02\x07\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x07\x08\n\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x07\r\x0e\n!\n\x03\x04\0\t\x12\x03\n\
    \x0b\x14\x1a\x15\x20User-Level\x20Metadata\n\n\x0b\n\x04\x04\0\t\x01\x12\
    \x03\n\x0b\x13\n\x0c\n\x05\x04\0\t\x01\x01\x12\x03\n\x0b\r\n\x0c\n\x05\
    \x04\0\t\x01\x02\x12\x03\n\x11\x13\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x0b\
    \x02\x14\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x0b\x02\n\x14\n\x0c\n\x05\
    \x04\0\x02\x01\x05\x12\x03\x0b\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\
    \x03\x0b\t\x0e\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0b\x11\x13\n\n\n\
    \x02\x04\x01\x12\x04\x0e\0\x10\x01\n\n\n\x03\x04\x01\x01\x12\x03\x0e\x08\
    \x12\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x0f\x02\"\n\x0c\n\x05\x04\x01\x02\
    \0\x04\x12\x03\x0f\x02\n\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x0f\x0b\
    \x15\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x0f\x16\x1d\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03\x0f\x20!b\x06proto3\
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
