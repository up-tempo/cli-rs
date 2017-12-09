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
pub struct AlbumInfo {
    // message fields
    pub id: i32,
    pub title: ::std::string::String,
    pub cover: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AlbumInfo {}

impl AlbumInfo {
    pub fn new() -> AlbumInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AlbumInfo {
        static mut instance: ::protobuf::lazy::Lazy<AlbumInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AlbumInfo,
        };
        unsafe {
            instance.get(AlbumInfo::new)
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

    // bytes cover = 12;

    pub fn clear_cover(&mut self) {
        self.cover.clear();
    }

    // Param is passed by value, moved
    pub fn set_cover(&mut self, v: ::std::vec::Vec<u8>) {
        self.cover = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cover(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.cover
    }

    // Take field
    pub fn take_cover(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.cover, ::std::vec::Vec::new())
    }

    pub fn get_cover(&self) -> &[u8] {
        &self.cover
    }

    fn get_cover_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.cover
    }

    fn mut_cover_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.cover
    }
}

impl ::protobuf::Message for AlbumInfo {
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
                12 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.cover)?;
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
        if !self.cover.is_empty() {
            my_size += ::protobuf::rt::bytes_size(12, &self.cover);
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
        if !self.cover.is_empty() {
            os.write_bytes(12, &self.cover)?;
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

impl ::protobuf::MessageStatic for AlbumInfo {
    fn new() -> AlbumInfo {
        AlbumInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<AlbumInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "id",
                    AlbumInfo::get_id_for_reflect,
                    AlbumInfo::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    AlbumInfo::get_title_for_reflect,
                    AlbumInfo::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "cover",
                    AlbumInfo::get_cover_for_reflect,
                    AlbumInfo::mut_cover_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AlbumInfo>(
                    "AlbumInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AlbumInfo {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_title();
        self.clear_cover();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AlbumInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AlbumInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AlbumList {
    // message fields
    pub albums: ::protobuf::RepeatedField<AlbumInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AlbumList {}

impl AlbumList {
    pub fn new() -> AlbumList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AlbumList {
        static mut instance: ::protobuf::lazy::Lazy<AlbumList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AlbumList,
        };
        unsafe {
            instance.get(AlbumList::new)
        }
    }

    // repeated .uptempo.AlbumInfo albums = 1;

    pub fn clear_albums(&mut self) {
        self.albums.clear();
    }

    // Param is passed by value, moved
    pub fn set_albums(&mut self, v: ::protobuf::RepeatedField<AlbumInfo>) {
        self.albums = v;
    }

    // Mutable pointer to the field.
    pub fn mut_albums(&mut self) -> &mut ::protobuf::RepeatedField<AlbumInfo> {
        &mut self.albums
    }

    // Take field
    pub fn take_albums(&mut self) -> ::protobuf::RepeatedField<AlbumInfo> {
        ::std::mem::replace(&mut self.albums, ::protobuf::RepeatedField::new())
    }

    pub fn get_albums(&self) -> &[AlbumInfo] {
        &self.albums
    }

    fn get_albums_for_reflect(&self) -> &::protobuf::RepeatedField<AlbumInfo> {
        &self.albums
    }

    fn mut_albums_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AlbumInfo> {
        &mut self.albums
    }
}

impl ::protobuf::Message for AlbumList {
    fn is_initialized(&self) -> bool {
        for v in &self.albums {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.albums)?;
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
        for value in &self.albums {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.albums {
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

impl ::protobuf::MessageStatic for AlbumList {
    fn new() -> AlbumList {
        AlbumList::new()
    }

    fn descriptor_static(_: ::std::option::Option<AlbumList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AlbumInfo>>(
                    "albums",
                    AlbumList::get_albums_for_reflect,
                    AlbumList::mut_albums_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AlbumList>(
                    "AlbumList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AlbumList {
    fn clear(&mut self) {
        self.clear_albums();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AlbumList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AlbumList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0balbum.proto\x12\x07uptempo\"S\n\tAlbumInfo\x12\x0e\n\x02id\x18\x01\
    \x20\x01(\x05R\x02id\x12\x14\n\x05title\x18\x0b\x20\x01(\tR\x05title\x12\
    \x14\n\x05cover\x18\x0c\x20\x01(\x0cR\x05coverJ\x04\x08\x02\x10\x0bJ\x04\
    \x08\r\x10\x1f\"7\n\tAlbumList\x12*\n\x06albums\x18\x01\x20\x03(\x0b2\
    \x12.uptempo.AlbumInfoR\x06albumsJ\xfd\x03\n\x06\x12\x04\0\0\x11\x01\n\
    \x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x0f\n\n\n\
    \x02\x04\0\x12\x04\x04\0\r\x01\n\n\n\x03\x04\0\x01\x12\x03\x04\x08\x11\n\
    #\n\x03\x04\0\t\x12\x03\x06\x0b\x13\x1a\x17\x20System-Level\x20Metadata\
    \n\n\x0b\n\x04\x04\0\t\0\x12\x03\x06\x0b\x12\n\x0c\n\x05\x04\0\t\0\x01\
    \x12\x03\x06\x0b\x0c\n\x0c\n\x05\x04\0\t\0\x02\x12\x03\x06\x10\x12\n\x0b\
    \n\x04\x04\0\x02\0\x12\x03\x07\x02\x0f\n\r\n\x05\x04\0\x02\0\x04\x12\x04\
    \x07\x02\x06\x13\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x07\x02\x07\n\x0c\n\
    \x05\x04\0\x02\0\x01\x12\x03\x07\x08\n\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03\x07\r\x0e\n!\n\x03\x04\0\t\x12\x03\n\x0b\x14\x1a\x15\x20User-Level\
    \x20Metadata\n\n\x0b\n\x04\x04\0\t\x01\x12\x03\n\x0b\x13\n\x0c\n\x05\x04\
    \0\t\x01\x01\x12\x03\n\x0b\r\n\x0c\n\x05\x04\0\t\x01\x02\x12\x03\n\x11\
    \x13\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x0b\x02\x14\n\r\n\x05\x04\0\x02\
    \x01\x04\x12\x04\x0b\x02\n\x14\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x0b\
    \x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0b\t\x0e\n\x0c\n\x05\x04\
    \0\x02\x01\x03\x12\x03\x0b\x11\x13\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x0c\
    \x02\x13\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\x0c\x02\x0b\x14\n\x0c\n\x05\
    \x04\0\x02\x02\x05\x12\x03\x0c\x02\x07\n\x0c\n\x05\x04\0\x02\x02\x01\x12\
    \x03\x0c\x08\r\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x0c\x10\x12\n\n\n\
    \x02\x04\x01\x12\x04\x0f\0\x11\x01\n\n\n\x03\x04\x01\x01\x12\x03\x0f\x08\
    \x11\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x10\x02\x20\n\x0c\n\x05\x04\x01\
    \x02\0\x04\x12\x03\x10\x02\n\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x10\
    \x0b\x14\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x10\x15\x1b\n\x0c\n\x05\
    \x04\x01\x02\0\x03\x12\x03\x10\x1e\x1fb\x06proto3\
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
