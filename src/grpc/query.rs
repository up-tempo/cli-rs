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
pub struct Query {
    // message fields
    pub page_number: i32,
    pub result_per_page: i32,
    // message oneof groups
    query_param: ::std::option::Option<Query_oneof_query_param>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Query {}

#[derive(Clone,PartialEq)]
pub enum Query_oneof_query_param {
    query(::std::string::String),
    id(i32),
}

impl Query {
    pub fn new() -> Query {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Query {
        static mut instance: ::protobuf::lazy::Lazy<Query> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Query,
        };
        unsafe {
            instance.get(Query::new)
        }
    }

    // string query = 1;

    pub fn clear_query(&mut self) {
        self.query_param = ::std::option::Option::None;
    }

    pub fn has_query(&self) -> bool {
        match self.query_param {
            ::std::option::Option::Some(Query_oneof_query_param::query(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_query(&mut self, v: ::std::string::String) {
        self.query_param = ::std::option::Option::Some(Query_oneof_query_param::query(v))
    }

    // Mutable pointer to the field.
    pub fn mut_query(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Query_oneof_query_param::query(_)) = self.query_param {
        } else {
            self.query_param = ::std::option::Option::Some(Query_oneof_query_param::query(::std::string::String::new()));
        }
        match self.query_param {
            ::std::option::Option::Some(Query_oneof_query_param::query(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_query(&mut self) -> ::std::string::String {
        if self.has_query() {
            match self.query_param.take() {
                ::std::option::Option::Some(Query_oneof_query_param::query(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_query(&self) -> &str {
        match self.query_param {
            ::std::option::Option::Some(Query_oneof_query_param::query(ref v)) => v,
            _ => "",
        }
    }

    // int32 id = 2;

    pub fn clear_id(&mut self) {
        self.query_param = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        match self.query_param {
            ::std::option::Option::Some(Query_oneof_query_param::id(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i32) {
        self.query_param = ::std::option::Option::Some(Query_oneof_query_param::id(v))
    }

    pub fn get_id(&self) -> i32 {
        match self.query_param {
            ::std::option::Option::Some(Query_oneof_query_param::id(v)) => v,
            _ => 0,
        }
    }

    // int32 page_number = 11;

    pub fn clear_page_number(&mut self) {
        self.page_number = 0;
    }

    // Param is passed by value, moved
    pub fn set_page_number(&mut self, v: i32) {
        self.page_number = v;
    }

    pub fn get_page_number(&self) -> i32 {
        self.page_number
    }

    fn get_page_number_for_reflect(&self) -> &i32 {
        &self.page_number
    }

    fn mut_page_number_for_reflect(&mut self) -> &mut i32 {
        &mut self.page_number
    }

    // int32 result_per_page = 12;

    pub fn clear_result_per_page(&mut self) {
        self.result_per_page = 0;
    }

    // Param is passed by value, moved
    pub fn set_result_per_page(&mut self, v: i32) {
        self.result_per_page = v;
    }

    pub fn get_result_per_page(&self) -> i32 {
        self.result_per_page
    }

    fn get_result_per_page_for_reflect(&self) -> &i32 {
        &self.result_per_page
    }

    fn mut_result_per_page_for_reflect(&mut self) -> &mut i32 {
        &mut self.result_per_page
    }
}

impl ::protobuf::Message for Query {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.query_param = ::std::option::Option::Some(Query_oneof_query_param::query(is.read_string()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.query_param = ::std::option::Option::Some(Query_oneof_query_param::id(is.read_int32()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.page_number = tmp;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.result_per_page = tmp;
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
        if self.page_number != 0 {
            my_size += ::protobuf::rt::value_size(11, self.page_number, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.result_per_page != 0 {
            my_size += ::protobuf::rt::value_size(12, self.result_per_page, ::protobuf::wire_format::WireTypeVarint);
        }
        if let ::std::option::Option::Some(ref v) = self.query_param {
            match v {
                &Query_oneof_query_param::query(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
                &Query_oneof_query_param::id(v) => {
                    my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.page_number != 0 {
            os.write_int32(11, self.page_number)?;
        }
        if self.result_per_page != 0 {
            os.write_int32(12, self.result_per_page)?;
        }
        if let ::std::option::Option::Some(ref v) = self.query_param {
            match v {
                &Query_oneof_query_param::query(ref v) => {
                    os.write_string(1, v)?;
                },
                &Query_oneof_query_param::id(v) => {
                    os.write_int32(2, v)?;
                },
            };
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

impl ::protobuf::MessageStatic for Query {
    fn new() -> Query {
        Query::new()
    }

    fn descriptor_static(_: ::std::option::Option<Query>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "query",
                    Query::has_query,
                    Query::get_query,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor::<_>(
                    "id",
                    Query::has_id,
                    Query::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "page_number",
                    Query::get_page_number_for_reflect,
                    Query::mut_page_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "result_per_page",
                    Query::get_result_per_page_for_reflect,
                    Query::mut_result_per_page_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Query>(
                    "Query",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Query {
    fn clear(&mut self) {
        self.clear_query();
        self.clear_id();
        self.clear_page_number();
        self.clear_result_per_page();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Query {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Query {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AggregateList {
    // message fields
    pub playlists: ::protobuf::SingularPtrField<super::playlist::PlaylistList>,
    pub songs: ::protobuf::SingularPtrField<super::song::SongList>,
    pub albums: ::protobuf::SingularPtrField<super::album::AlbumList>,
    pub artists: ::protobuf::SingularPtrField<super::artist::ArtistList>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AggregateList {}

impl AggregateList {
    pub fn new() -> AggregateList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AggregateList {
        static mut instance: ::protobuf::lazy::Lazy<AggregateList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AggregateList,
        };
        unsafe {
            instance.get(AggregateList::new)
        }
    }

    // .uptempo.PlaylistList playlists = 1;

    pub fn clear_playlists(&mut self) {
        self.playlists.clear();
    }

    pub fn has_playlists(&self) -> bool {
        self.playlists.is_some()
    }

    // Param is passed by value, moved
    pub fn set_playlists(&mut self, v: super::playlist::PlaylistList) {
        self.playlists = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_playlists(&mut self) -> &mut super::playlist::PlaylistList {
        if self.playlists.is_none() {
            self.playlists.set_default();
        }
        self.playlists.as_mut().unwrap()
    }

    // Take field
    pub fn take_playlists(&mut self) -> super::playlist::PlaylistList {
        self.playlists.take().unwrap_or_else(|| super::playlist::PlaylistList::new())
    }

    pub fn get_playlists(&self) -> &super::playlist::PlaylistList {
        self.playlists.as_ref().unwrap_or_else(|| super::playlist::PlaylistList::default_instance())
    }

    fn get_playlists_for_reflect(&self) -> &::protobuf::SingularPtrField<super::playlist::PlaylistList> {
        &self.playlists
    }

    fn mut_playlists_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::playlist::PlaylistList> {
        &mut self.playlists
    }

    // .uptempo.SongList songs = 2;

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

    // .uptempo.AlbumList albums = 3;

    pub fn clear_albums(&mut self) {
        self.albums.clear();
    }

    pub fn has_albums(&self) -> bool {
        self.albums.is_some()
    }

    // Param is passed by value, moved
    pub fn set_albums(&mut self, v: super::album::AlbumList) {
        self.albums = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_albums(&mut self) -> &mut super::album::AlbumList {
        if self.albums.is_none() {
            self.albums.set_default();
        }
        self.albums.as_mut().unwrap()
    }

    // Take field
    pub fn take_albums(&mut self) -> super::album::AlbumList {
        self.albums.take().unwrap_or_else(|| super::album::AlbumList::new())
    }

    pub fn get_albums(&self) -> &super::album::AlbumList {
        self.albums.as_ref().unwrap_or_else(|| super::album::AlbumList::default_instance())
    }

    fn get_albums_for_reflect(&self) -> &::protobuf::SingularPtrField<super::album::AlbumList> {
        &self.albums
    }

    fn mut_albums_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::album::AlbumList> {
        &mut self.albums
    }

    // .uptempo.ArtistList artists = 4;

    pub fn clear_artists(&mut self) {
        self.artists.clear();
    }

    pub fn has_artists(&self) -> bool {
        self.artists.is_some()
    }

    // Param is passed by value, moved
    pub fn set_artists(&mut self, v: super::artist::ArtistList) {
        self.artists = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_artists(&mut self) -> &mut super::artist::ArtistList {
        if self.artists.is_none() {
            self.artists.set_default();
        }
        self.artists.as_mut().unwrap()
    }

    // Take field
    pub fn take_artists(&mut self) -> super::artist::ArtistList {
        self.artists.take().unwrap_or_else(|| super::artist::ArtistList::new())
    }

    pub fn get_artists(&self) -> &super::artist::ArtistList {
        self.artists.as_ref().unwrap_or_else(|| super::artist::ArtistList::default_instance())
    }

    fn get_artists_for_reflect(&self) -> &::protobuf::SingularPtrField<super::artist::ArtistList> {
        &self.artists
    }

    fn mut_artists_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::artist::ArtistList> {
        &mut self.artists
    }
}

impl ::protobuf::Message for AggregateList {
    fn is_initialized(&self) -> bool {
        for v in &self.playlists {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.songs {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.albums {
            if !v.is_initialized() {
                return false;
            }
        };
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.playlists)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.songs)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.albums)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.artists)?;
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
        if let Some(ref v) = self.playlists.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.songs.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.albums.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.artists.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.playlists.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.songs.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.albums.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.artists.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for AggregateList {
    fn new() -> AggregateList {
        AggregateList::new()
    }

    fn descriptor_static(_: ::std::option::Option<AggregateList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist::PlaylistList>>(
                    "playlists",
                    AggregateList::get_playlists_for_reflect,
                    AggregateList::mut_playlists_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::song::SongList>>(
                    "songs",
                    AggregateList::get_songs_for_reflect,
                    AggregateList::mut_songs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::album::AlbumList>>(
                    "albums",
                    AggregateList::get_albums_for_reflect,
                    AggregateList::mut_albums_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::artist::ArtistList>>(
                    "artists",
                    AggregateList::get_artists_for_reflect,
                    AggregateList::mut_artists_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AggregateList>(
                    "AggregateList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AggregateList {
    fn clear(&mut self) {
        self.clear_playlists();
        self.clear_songs();
        self.clear_albums();
        self.clear_artists();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AggregateList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AggregateList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bquery.proto\x12\x07uptempo\x1a\x0balbum.proto\x1a\x0cartist.proto\
    \x1a\nsong.proto\x1a\x0eplaylist.proto\"\x95\x01\n\x05Query\x12\x16\n\
    \x05query\x18\x01\x20\x01(\tH\0R\x05query\x12\x10\n\x02id\x18\x02\x20\
    \x01(\x05H\0R\x02id\x12\x1f\n\x0bpage_number\x18\x0b\x20\x01(\x05R\npage\
    Number\x12&\n\x0fresult_per_page\x18\x0c\x20\x01(\x05R\rresultPerPageB\r\
    \n\x0bquery_paramJ\x04\x08\x03\x10\x0bJ\x04\x08\r\x10\x15\"\xc8\x01\n\rA\
    ggregateList\x123\n\tplaylists\x18\x01\x20\x01(\x0b2\x15.uptempo.Playlis\
    tListR\tplaylists\x12'\n\x05songs\x18\x02\x20\x01(\x0b2\x11.uptempo.Song\
    ListR\x05songs\x12*\n\x06albums\x18\x03\x20\x01(\x0b2\x12.uptempo.AlbumL\
    istR\x06albums\x12-\n\x07artists\x18\x04\x20\x01(\x0b2\x13.uptempo.Artis\
    tListR\x07artists2\x9e\x02\n\x0cQueryService\x129\n\x0eQueryPlaylists\
    \x12\x0e.uptempo.Query\x1a\x15.uptempo.PlaylistList\"\0\x121\n\nQuerySon\
    gs\x12\x0e.uptempo.Query\x1a\x11.uptempo.SongList\"\0\x123\n\x0bQueryAlb\
    ums\x12\x0e.uptempo.Query\x1a\x12.uptempo.AlbumList\"\0\x125\n\x0cQueryA\
    rtists\x12\x0e.uptempo.Query\x1a\x13.uptempo.ArtistList\"\0\x124\n\x08Qu\
    eryAll\x12\x0e.uptempo.Query\x1a\x16.uptempo.AggregateList\"\0J\xe4\x08\
    \n\x06\x12\x04\0\0$\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\
    \x12\x03\x02\x08\x0f\n\t\n\x02\x03\0\x12\x03\x04\x07\x14\n\t\n\x02\x03\
    \x01\x12\x03\x05\x07\x15\n\t\n\x02\x03\x02\x12\x03\x06\x07\x13\n\t\n\x02\
    \x03\x03\x12\x03\x07\x07\x17\n\n\n\x02\x06\0\x12\x04\t\0\x0f\x01\n\n\n\
    \x03\x06\0\x01\x12\x03\t\x08\x14\n\x0b\n\x04\x06\0\x02\0\x12\x03\n\x025\
    \n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\n\x06\x14\n\x0c\n\x05\x06\0\x02\0\
    \x02\x12\x03\n\x15\x1a\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\n%1\n\x0b\n\
    \x04\x06\0\x02\x01\x12\x03\x0b\x02-\n\x0c\n\x05\x06\0\x02\x01\x01\x12\
    \x03\x0b\x06\x10\n\x0c\n\x05\x06\0\x02\x01\x02\x12\x03\x0b\x11\x16\n\x0c\
    \n\x05\x06\0\x02\x01\x03\x12\x03\x0b!)\n\x0b\n\x04\x06\0\x02\x02\x12\x03\
    \x0c\x02/\n\x0c\n\x05\x06\0\x02\x02\x01\x12\x03\x0c\x06\x11\n\x0c\n\x05\
    \x06\0\x02\x02\x02\x12\x03\x0c\x12\x17\n\x0c\n\x05\x06\0\x02\x02\x03\x12\
    \x03\x0c\"+\n\x0b\n\x04\x06\0\x02\x03\x12\x03\r\x021\n\x0c\n\x05\x06\0\
    \x02\x03\x01\x12\x03\r\x06\x12\n\x0c\n\x05\x06\0\x02\x03\x02\x12\x03\r\
    \x13\x18\n\x0c\n\x05\x06\0\x02\x03\x03\x12\x03\r#-\n\x0b\n\x04\x06\0\x02\
    \x04\x12\x03\x0e\x020\n\x0c\n\x05\x06\0\x02\x04\x01\x12\x03\x0e\x06\x0e\
    \n\x0c\n\x05\x06\0\x02\x04\x02\x12\x03\x0e\x0f\x14\n\x0c\n\x05\x06\0\x02\
    \x04\x03\x12\x03\x0e\x1f,\n\n\n\x02\x04\0\x12\x04\x11\0\x1d\x01\n\n\n\
    \x03\x04\0\x01\x12\x03\x11\x08\r\n\x1e\n\x03\x04\0\t\x12\x03\x13\x0b\x13\
    \x1a\x12\x20Query\x20Parameters\n\n\x0b\n\x04\x04\0\t\0\x12\x03\x13\x0b\
    \x12\n\x0c\n\x05\x04\0\t\0\x01\x12\x03\x13\x0b\x0c\n\x0c\n\x05\x04\0\t\0\
    \x02\x12\x03\x13\x10\x12\n\x0c\n\x04\x04\0\x08\0\x12\x04\x14\x02\x17\x03\
    \n\x0c\n\x05\x04\0\x08\0\x01\x12\x03\x14\x08\x13\n\x0b\n\x04\x04\0\x02\0\
    \x12\x03\x15\x04\x15\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x15\x04\n\n\x0c\
    \n\x05\x04\0\x02\0\x01\x12\x03\x15\x0b\x10\n\x0c\n\x05\x04\0\x02\0\x03\
    \x12\x03\x15\x13\x14\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x16\x04\x11\n\x0c\
    \n\x05\x04\0\x02\x01\x05\x12\x03\x16\x04\t\n\x0c\n\x05\x04\0\x02\x01\x01\
    \x12\x03\x16\n\x0c\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x16\x0f\x10\n\
    \x1f\n\x03\x04\0\t\x12\x03\x1a\x0b\x14\x1a\x13\x20Return\x20Parameters\n\
    \n\x0b\n\x04\x04\0\t\x01\x12\x03\x1a\x0b\x13\n\x0c\n\x05\x04\0\t\x01\x01\
    \x12\x03\x1a\x0b\r\n\x0c\n\x05\x04\0\t\x01\x02\x12\x03\x1a\x11\x13\n\x0b\
    \n\x04\x04\0\x02\x02\x12\x03\x1b\x02\x19\n\r\n\x05\x04\0\x02\x02\x04\x12\
    \x04\x1b\x02\x1a\x14\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x1b\x02\x07\n\
    \x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x1b\x08\x13\n\x0c\n\x05\x04\0\x02\
    \x02\x03\x12\x03\x1b\x16\x18\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x1c\x02\
    \x1d\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\x1c\x02\x1b\x19\n\x0c\n\x05\x04\
    \0\x02\x03\x05\x12\x03\x1c\x02\x07\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\
    \x1c\x08\x17\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x1c\x1a\x1c\n\n\n\x02\
    \x04\x01\x12\x04\x1f\0$\x01\n\n\n\x03\x04\x01\x01\x12\x03\x1f\x08\x15\n\
    \x0b\n\x04\x04\x01\x02\0\x12\x03\x20\x02\x1d\n\r\n\x05\x04\x01\x02\0\x04\
    \x12\x04\x20\x02\x1f\x17\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x20\x02\
    \x0e\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x20\x0f\x18\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03\x20\x1b\x1c\n\x0b\n\x04\x04\x01\x02\x01\x12\x03!\
    \x02\x15\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04!\x02\x20\x1d\n\x0c\n\x05\
    \x04\x01\x02\x01\x06\x12\x03!\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\
    \x03!\x0b\x10\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03!\x13\x14\n\x0b\n\
    \x04\x04\x01\x02\x02\x12\x03\"\x02\x17\n\r\n\x05\x04\x01\x02\x02\x04\x12\
    \x04\"\x02!\x15\n\x0c\n\x05\x04\x01\x02\x02\x06\x12\x03\"\x02\x0b\n\x0c\
    \n\x05\x04\x01\x02\x02\x01\x12\x03\"\x0c\x12\n\x0c\n\x05\x04\x01\x02\x02\
    \x03\x12\x03\"\x15\x16\n\x0b\n\x04\x04\x01\x02\x03\x12\x03#\x02\x19\n\r\
    \n\x05\x04\x01\x02\x03\x04\x12\x04#\x02\"\x17\n\x0c\n\x05\x04\x01\x02\
    \x03\x06\x12\x03#\x02\x0c\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03#\r\x14\
    \n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x03#\x17\x18b\x06proto3\
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
