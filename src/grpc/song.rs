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
pub struct SongInfo {
    // message fields
    pub id: i32,
    pub filetype: ::std::string::String,
    pub title: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SongInfo {}

impl SongInfo {
    pub fn new() -> SongInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SongInfo {
        static mut instance: ::protobuf::lazy::Lazy<SongInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SongInfo,
        };
        unsafe {
            instance.get(SongInfo::new)
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

    // string filetype = 2;

    pub fn clear_filetype(&mut self) {
        self.filetype.clear();
    }

    // Param is passed by value, moved
    pub fn set_filetype(&mut self, v: ::std::string::String) {
        self.filetype = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filetype(&mut self) -> &mut ::std::string::String {
        &mut self.filetype
    }

    // Take field
    pub fn take_filetype(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.filetype, ::std::string::String::new())
    }

    pub fn get_filetype(&self) -> &str {
        &self.filetype
    }

    fn get_filetype_for_reflect(&self) -> &::std::string::String {
        &self.filetype
    }

    fn mut_filetype_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.filetype
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

impl ::protobuf::Message for SongInfo {
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
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.filetype)?;
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
        if !self.filetype.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.filetype);
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
        if !self.filetype.is_empty() {
            os.write_string(2, &self.filetype)?;
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

impl ::protobuf::MessageStatic for SongInfo {
    fn new() -> SongInfo {
        SongInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<SongInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "id",
                    SongInfo::get_id_for_reflect,
                    SongInfo::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "filetype",
                    SongInfo::get_filetype_for_reflect,
                    SongInfo::mut_filetype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    SongInfo::get_title_for_reflect,
                    SongInfo::mut_title_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SongInfo>(
                    "SongInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SongInfo {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_filetype();
        self.clear_title();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SongInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SongInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SongChunk {
    // message fields
    pub id: i32,
    pub chunk_data: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SongChunk {}

impl SongChunk {
    pub fn new() -> SongChunk {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SongChunk {
        static mut instance: ::protobuf::lazy::Lazy<SongChunk> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SongChunk,
        };
        unsafe {
            instance.get(SongChunk::new)
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

    // bytes chunk_data = 2;

    pub fn clear_chunk_data(&mut self) {
        self.chunk_data.clear();
    }

    // Param is passed by value, moved
    pub fn set_chunk_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.chunk_data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_chunk_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.chunk_data
    }

    // Take field
    pub fn take_chunk_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.chunk_data, ::std::vec::Vec::new())
    }

    pub fn get_chunk_data(&self) -> &[u8] {
        &self.chunk_data
    }

    fn get_chunk_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.chunk_data
    }

    fn mut_chunk_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.chunk_data
    }
}

impl ::protobuf::Message for SongChunk {
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
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.chunk_data)?;
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
        if !self.chunk_data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.chunk_data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_int32(1, self.id)?;
        }
        if !self.chunk_data.is_empty() {
            os.write_bytes(2, &self.chunk_data)?;
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

impl ::protobuf::MessageStatic for SongChunk {
    fn new() -> SongChunk {
        SongChunk::new()
    }

    fn descriptor_static(_: ::std::option::Option<SongChunk>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "id",
                    SongChunk::get_id_for_reflect,
                    SongChunk::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "chunk_data",
                    SongChunk::get_chunk_data_for_reflect,
                    SongChunk::mut_chunk_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SongChunk>(
                    "SongChunk",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SongChunk {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_chunk_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SongChunk {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SongChunk {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SongList {
    // message fields
    pub songs: ::protobuf::RepeatedField<SongInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SongList {}

impl SongList {
    pub fn new() -> SongList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SongList {
        static mut instance: ::protobuf::lazy::Lazy<SongList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SongList,
        };
        unsafe {
            instance.get(SongList::new)
        }
    }

    // repeated .uptempo.SongInfo songs = 1;

    pub fn clear_songs(&mut self) {
        self.songs.clear();
    }

    // Param is passed by value, moved
    pub fn set_songs(&mut self, v: ::protobuf::RepeatedField<SongInfo>) {
        self.songs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_songs(&mut self) -> &mut ::protobuf::RepeatedField<SongInfo> {
        &mut self.songs
    }

    // Take field
    pub fn take_songs(&mut self) -> ::protobuf::RepeatedField<SongInfo> {
        ::std::mem::replace(&mut self.songs, ::protobuf::RepeatedField::new())
    }

    pub fn get_songs(&self) -> &[SongInfo] {
        &self.songs
    }

    fn get_songs_for_reflect(&self) -> &::protobuf::RepeatedField<SongInfo> {
        &self.songs
    }

    fn mut_songs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<SongInfo> {
        &mut self.songs
    }
}

impl ::protobuf::Message for SongList {
    fn is_initialized(&self) -> bool {
        for v in &self.songs {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.songs)?;
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
        for value in &self.songs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.songs {
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

impl ::protobuf::MessageStatic for SongList {
    fn new() -> SongList {
        SongList::new()
    }

    fn descriptor_static(_: ::std::option::Option<SongList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SongInfo>>(
                    "songs",
                    SongList::get_songs_for_reflect,
                    SongList::mut_songs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SongList>(
                    "SongList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SongList {
    fn clear(&mut self) {
        self.clear_songs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SongList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SongList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nsong.proto\x12\x07uptempo\"X\n\x08SongInfo\x12\x0e\n\x02id\x18\x01\
    \x20\x01(\x05R\x02id\x12\x1a\n\x08filetype\x18\x02\x20\x01(\tR\x08filety\
    pe\x12\x14\n\x05title\x18\x0b\x20\x01(\tR\x05titleJ\x04\x08\x03\x10\x0bJ\
    \x04\x08\x0c\x10\x1f\":\n\tSongChunk\x12\x0e\n\x02id\x18\x01\x20\x01(\
    \x05R\x02id\x12\x1d\n\nchunk_data\x18\x02\x20\x01(\x0cR\tchunkData\"3\n\
    \x08SongList\x12'\n\x05songs\x18\x01\x20\x03(\x0b2\x11.uptempo.SongInfoR\
    \x05songs2G\n\x0bSongService\x128\n\x0bGetSongData\x12\x11.uptempo.SongI\
    nfo\x1a\x12.uptempo.SongChunk\"\00\x01J\xfe\x05\n\x06\x12\x04\0\0\x1a\
    \x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x0f\
    \n\n\n\x02\x06\0\x12\x04\x04\0\x06\x01\n\n\n\x03\x06\0\x01\x12\x03\x04\
    \x08\x13\n\x0b\n\x04\x06\0\x02\0\x12\x03\x05\x029\n\x0c\n\x05\x06\0\x02\
    \0\x01\x12\x03\x05\x06\x11\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x05\x12\
    \x1a\n\x0c\n\x05\x06\0\x02\0\x06\x12\x03\x05%+\n\x0c\n\x05\x06\0\x02\0\
    \x03\x12\x03\x05,5\n\n\n\x02\x04\0\x12\x04\x08\0\x11\x01\n\n\n\x03\x04\0\
    \x01\x12\x03\x08\x08\x10\n#\n\x03\x04\0\t\x12\x03\n\x0b\x13\x1a\x17\x20S\
    ystem-Level\x20Metadata\n\n\x0b\n\x04\x04\0\t\0\x12\x03\n\x0b\x12\n\x0c\
    \n\x05\x04\0\t\0\x01\x12\x03\n\x0b\x0c\n\x0c\n\x05\x04\0\t\0\x02\x12\x03\
    \n\x10\x12\n\x0b\n\x04\x04\0\x02\0\x12\x03\x0b\x02\x0f\n\r\n\x05\x04\0\
    \x02\0\x04\x12\x04\x0b\x02\n\x13\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x0b\
    \x02\x07\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0b\x08\n\n\x0c\n\x05\x04\0\
    \x02\0\x03\x12\x03\x0b\r\x0e\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x0c\x02\
    \x16\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x0c\x02\x0b\x0f\n\x0c\n\x05\x04\
    \0\x02\x01\x05\x12\x03\x0c\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\
    \x0c\t\x11\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0c\x14\x15\n!\n\x03\
    \x04\0\t\x12\x03\x0f\x0b\x14\x1a\x15\x20User-Level\x20Metadata\n\n\x0b\n\
    \x04\x04\0\t\x01\x12\x03\x0f\x0b\x13\n\x0c\n\x05\x04\0\t\x01\x01\x12\x03\
    \x0f\x0b\r\n\x0c\n\x05\x04\0\t\x01\x02\x12\x03\x0f\x11\x13\n\x0b\n\x04\
    \x04\0\x02\x02\x12\x03\x10\x02\x14\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\
    \x10\x02\x0f\x14\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x10\x02\x08\n\x0c\
    \n\x05\x04\0\x02\x02\x01\x12\x03\x10\t\x0e\n\x0c\n\x05\x04\0\x02\x02\x03\
    \x12\x03\x10\x11\x13\n\n\n\x02\x04\x01\x12\x04\x13\0\x16\x01\n\n\n\x03\
    \x04\x01\x01\x12\x03\x13\x08\x11\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x14\
    \x02\x0f\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x14\x02\x13\x13\n\x0c\n\x05\
    \x04\x01\x02\0\x05\x12\x03\x14\x02\x07\n\x0c\n\x05\x04\x01\x02\0\x01\x12\
    \x03\x14\x08\n\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x14\r\x0e\n\x0b\n\
    \x04\x04\x01\x02\x01\x12\x03\x15\x02\x17\n\r\n\x05\x04\x01\x02\x01\x04\
    \x12\x04\x15\x02\x14\x0f\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x15\x02\
    \x07\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x15\x08\x12\n\x0c\n\x05\x04\
    \x01\x02\x01\x03\x12\x03\x15\x15\x16\n\n\n\x02\x04\x02\x12\x04\x18\0\x1a\
    \x01\n\n\n\x03\x04\x02\x01\x12\x03\x18\x08\x10\n\x0b\n\x04\x04\x02\x02\0\
    \x12\x03\x19\x02\x1e\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03\x19\x02\n\n\
    \x0c\n\x05\x04\x02\x02\0\x06\x12\x03\x19\x0b\x13\n\x0c\n\x05\x04\x02\x02\
    \0\x01\x12\x03\x19\x14\x19\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x19\x1c\
    \x1db\x06proto3\
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
