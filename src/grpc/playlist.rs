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
pub struct PlaylistInfo {
    // message fields
    pub id: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PlaylistInfo {}

impl PlaylistInfo {
    pub fn new() -> PlaylistInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PlaylistInfo {
        static mut instance: ::protobuf::lazy::Lazy<PlaylistInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PlaylistInfo,
        };
        unsafe {
            instance.get(PlaylistInfo::new)
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
}

impl ::protobuf::Message for PlaylistInfo {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_int32(1, self.id)?;
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

impl ::protobuf::MessageStatic for PlaylistInfo {
    fn new() -> PlaylistInfo {
        PlaylistInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<PlaylistInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "id",
                    PlaylistInfo::get_id_for_reflect,
                    PlaylistInfo::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PlaylistInfo>(
                    "PlaylistInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PlaylistInfo {
    fn clear(&mut self) {
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PlaylistInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlaylistInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Playlist {
    // message fields
    pub name: ::std::string::String,
    pub description: ::std::string::String,
    pub songs: ::protobuf::SingularPtrField<super::song::SongList>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Playlist {}

impl Playlist {
    pub fn new() -> Playlist {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Playlist {
        static mut instance: ::protobuf::lazy::Lazy<Playlist> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Playlist,
        };
        unsafe {
            instance.get(Playlist::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // string description = 2;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description, ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    fn get_description_for_reflect(&self) -> &::std::string::String {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // .uptempo.SongList songs = 3;

    pub fn clear_songs(&mut self) {
        self.songs.clear();
    }

    pub fn has_songs(&self) -> bool {
        self.songs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_songs(&mut self, v: super::song::SongList) {
        self.songs = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_songs(&mut self) -> &mut super::song::SongList {
        if self.songs.is_none() {
            self.songs.set_default();
        }
        self.songs.as_mut().unwrap()
    }

    // Take field
    pub fn take_songs(&mut self) -> super::song::SongList {
        self.songs.take().unwrap_or_else(|| super::song::SongList::new())
    }

    pub fn get_songs(&self) -> &super::song::SongList {
        self.songs.as_ref().unwrap_or_else(|| super::song::SongList::default_instance())
    }

    fn get_songs_for_reflect(&self) -> &::protobuf::SingularPtrField<super::song::SongList> {
        &self.songs
    }

    fn mut_songs_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::song::SongList> {
        &mut self.songs
    }
}

impl ::protobuf::Message for Playlist {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.songs)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.description);
        }
        if let Some(ref v) = self.songs.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.description.is_empty() {
            os.write_string(2, &self.description)?;
        }
        if let Some(ref v) = self.songs.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for Playlist {
    fn new() -> Playlist {
        Playlist::new()
    }

    fn descriptor_static(_: ::std::option::Option<Playlist>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Playlist::get_name_for_reflect,
                    Playlist::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    Playlist::get_description_for_reflect,
                    Playlist::mut_description_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::song::SongList>>(
                    "songs",
                    Playlist::get_songs_for_reflect,
                    Playlist::mut_songs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Playlist>(
                    "Playlist",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Playlist {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_description();
        self.clear_songs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Playlist {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Playlist {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PlaylistList {
    // message fields
    pub playlists: ::protobuf::RepeatedField<PlaylistInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PlaylistList {}

impl PlaylistList {
    pub fn new() -> PlaylistList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PlaylistList {
        static mut instance: ::protobuf::lazy::Lazy<PlaylistList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PlaylistList,
        };
        unsafe {
            instance.get(PlaylistList::new)
        }
    }

    // repeated .uptempo.PlaylistInfo playlists = 1;

    pub fn clear_playlists(&mut self) {
        self.playlists.clear();
    }

    // Param is passed by value, moved
    pub fn set_playlists(&mut self, v: ::protobuf::RepeatedField<PlaylistInfo>) {
        self.playlists = v;
    }

    // Mutable pointer to the field.
    pub fn mut_playlists(&mut self) -> &mut ::protobuf::RepeatedField<PlaylistInfo> {
        &mut self.playlists
    }

    // Take field
    pub fn take_playlists(&mut self) -> ::protobuf::RepeatedField<PlaylistInfo> {
        ::std::mem::replace(&mut self.playlists, ::protobuf::RepeatedField::new())
    }

    pub fn get_playlists(&self) -> &[PlaylistInfo] {
        &self.playlists
    }

    fn get_playlists_for_reflect(&self) -> &::protobuf::RepeatedField<PlaylistInfo> {
        &self.playlists
    }

    fn mut_playlists_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PlaylistInfo> {
        &mut self.playlists
    }
}

impl ::protobuf::Message for PlaylistList {
    fn is_initialized(&self) -> bool {
        for v in &self.playlists {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.playlists)?;
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
        for value in &self.playlists {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.playlists {
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

impl ::protobuf::MessageStatic for PlaylistList {
    fn new() -> PlaylistList {
        PlaylistList::new()
    }

    fn descriptor_static(_: ::std::option::Option<PlaylistList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PlaylistInfo>>(
                    "playlists",
                    PlaylistList::get_playlists_for_reflect,
                    PlaylistList::mut_playlists_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PlaylistList>(
                    "PlaylistList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PlaylistList {
    fn clear(&mut self) {
        self.clear_playlists();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PlaylistList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlaylistList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0eplaylist.proto\x12\x07uptempo\x1a\nsong.proto\"\x1e\n\x0cPlaylistI\
    nfo\x12\x0e\n\x02id\x18\x01\x20\x01(\x05R\x02id\"i\n\x08Playlist\x12\x12\
    \n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x20\n\x0bdescription\x18\x02\
    \x20\x01(\tR\x0bdescription\x12'\n\x05songs\x18\x03\x20\x01(\x0b2\x11.up\
    tempo.SongListR\x05songs\"C\n\x0cPlaylistList\x123\n\tplaylists\x18\x01\
    \x20\x03(\x0b2\x15.uptempo.PlaylistInfoR\tplaylists2L\n\x0fPlaylistServi\
    ce\x129\n\x0bGetPlaylist\x12\x15.uptempo.PlaylistInfo\x1a\x11.uptempo.Pl\
    aylist\"\0J\x9b\x04\n\x06\x12\x04\0\0\x16\x01\n\x08\n\x01\x0c\x12\x03\0\
    \0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x0f\n\t\n\x02\x03\0\x12\x03\x04\
    \x07\x13\n\n\n\x02\x06\0\x12\x04\x06\0\x08\x01\n\n\n\x03\x06\0\x01\x12\
    \x03\x06\x08\x17\n\x0b\n\x04\x06\0\x02\0\x12\x03\x07\x025\n\x0c\n\x05\
    \x06\0\x02\0\x01\x12\x03\x07\x06\x11\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\
    \x07\x12\x1e\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x07)1\n\n\n\x02\x04\0\
    \x12\x04\n\0\x0c\x01\n\n\n\x03\x04\0\x01\x12\x03\n\x08\x14\n\x0b\n\x04\
    \x04\0\x02\0\x12\x03\x0b\x02\x0f\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x0b\
    \x02\n\x16\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x0b\x02\x07\n\x0c\n\x05\
    \x04\0\x02\0\x01\x12\x03\x0b\x08\n\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\
    \x0b\r\x0e\n\n\n\x02\x04\x01\x12\x04\x0e\0\x12\x01\n\n\n\x03\x04\x01\x01\
    \x12\x03\x0e\x08\x10\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x0f\x02\x12\n\r\n\
    \x05\x04\x01\x02\0\x04\x12\x04\x0f\x02\x0e\x12\n\x0c\n\x05\x04\x01\x02\0\
    \x05\x12\x03\x0f\x02\x08\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x0f\t\r\n\
    \x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x0f\x10\x11\n\x0b\n\x04\x04\x01\x02\
    \x01\x12\x03\x10\x02\x19\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x10\x02\
    \x0f\x12\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x10\x02\x08\n\x0c\n\x05\
    \x04\x01\x02\x01\x01\x12\x03\x10\t\x14\n\x0c\n\x05\x04\x01\x02\x01\x03\
    \x12\x03\x10\x17\x18\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x11\x02\x15\n\r\
    \n\x05\x04\x01\x02\x02\x04\x12\x04\x11\x02\x10\x19\n\x0c\n\x05\x04\x01\
    \x02\x02\x06\x12\x03\x11\x02\n\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\
    \x11\x0b\x10\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x11\x13\x14\n\n\n\
    \x02\x04\x02\x12\x04\x14\0\x16\x01\n\n\n\x03\x04\x02\x01\x12\x03\x14\x08\
    \x14\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x15\x02&\n\x0c\n\x05\x04\x02\x02\
    \0\x04\x12\x03\x15\x02\n\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03\x15\x0b\
    \x17\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x15\x18!\n\x0c\n\x05\x04\x02\
    \x02\0\x03\x12\x03\x15$%b\x06proto3\
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
