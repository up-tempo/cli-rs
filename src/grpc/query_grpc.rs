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

const METHOD_QUERY_SERVICE_QUERY_PLAYLISTS: ::grpcio::Method<super::query::Query, super::playlist::PlaylistList> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/uptempo.QueryService/QueryPlaylists",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_QUERY_SERVICE_QUERY_SONGS: ::grpcio::Method<super::query::Query, super::song::SongList> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/uptempo.QueryService/QuerySongs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_QUERY_SERVICE_QUERY_ALBUMS: ::grpcio::Method<super::query::Query, super::album::AlbumList> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/uptempo.QueryService/QueryAlbums",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_QUERY_SERVICE_QUERY_ARTISTS: ::grpcio::Method<super::query::Query, super::artist::ArtistList> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/uptempo.QueryService/QueryArtists",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_QUERY_SERVICE_QUERY_ALL: ::grpcio::Method<super::query::Query, super::query::AggregateList> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/uptempo.QueryService/QueryAll",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct QueryServiceClient {
    client: ::grpcio::Client,
}

impl QueryServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        QueryServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn query_playlists_opt(&self, req: super::query::Query, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::playlist::PlaylistList> {
        self.client.unary_call(&METHOD_QUERY_SERVICE_QUERY_PLAYLISTS, req, opt)
    }

    pub fn query_playlists(&self, req: super::query::Query) -> ::grpcio::Result<super::playlist::PlaylistList> {
        self.query_playlists_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_playlists_async_opt(&self, req: super::query::Query, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::playlist::PlaylistList> {
        self.client.unary_call_async(&METHOD_QUERY_SERVICE_QUERY_PLAYLISTS, req, opt)
    }

    pub fn query_playlists_async(&self, req: super::query::Query) -> ::grpcio::ClientUnaryReceiver<super::playlist::PlaylistList> {
        self.query_playlists_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_songs_opt(&self, req: super::query::Query, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::song::SongList> {
        self.client.unary_call(&METHOD_QUERY_SERVICE_QUERY_SONGS, req, opt)
    }

    pub fn query_songs(&self, req: super::query::Query) -> ::grpcio::Result<super::song::SongList> {
        self.query_songs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_songs_async_opt(&self, req: super::query::Query, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::song::SongList> {
        self.client.unary_call_async(&METHOD_QUERY_SERVICE_QUERY_SONGS, req, opt)
    }

    pub fn query_songs_async(&self, req: super::query::Query) -> ::grpcio::ClientUnaryReceiver<super::song::SongList> {
        self.query_songs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_albums_opt(&self, req: super::query::Query, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::album::AlbumList> {
        self.client.unary_call(&METHOD_QUERY_SERVICE_QUERY_ALBUMS, req, opt)
    }

    pub fn query_albums(&self, req: super::query::Query) -> ::grpcio::Result<super::album::AlbumList> {
        self.query_albums_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_albums_async_opt(&self, req: super::query::Query, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::album::AlbumList> {
        self.client.unary_call_async(&METHOD_QUERY_SERVICE_QUERY_ALBUMS, req, opt)
    }

    pub fn query_albums_async(&self, req: super::query::Query) -> ::grpcio::ClientUnaryReceiver<super::album::AlbumList> {
        self.query_albums_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_artists_opt(&self, req: super::query::Query, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::artist::ArtistList> {
        self.client.unary_call(&METHOD_QUERY_SERVICE_QUERY_ARTISTS, req, opt)
    }

    pub fn query_artists(&self, req: super::query::Query) -> ::grpcio::Result<super::artist::ArtistList> {
        self.query_artists_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_artists_async_opt(&self, req: super::query::Query, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::artist::ArtistList> {
        self.client.unary_call_async(&METHOD_QUERY_SERVICE_QUERY_ARTISTS, req, opt)
    }

    pub fn query_artists_async(&self, req: super::query::Query) -> ::grpcio::ClientUnaryReceiver<super::artist::ArtistList> {
        self.query_artists_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_all_opt(&self, req: super::query::Query, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::query::AggregateList> {
        self.client.unary_call(&METHOD_QUERY_SERVICE_QUERY_ALL, req, opt)
    }

    pub fn query_all(&self, req: super::query::Query) -> ::grpcio::Result<super::query::AggregateList> {
        self.query_all_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_all_async_opt(&self, req: super::query::Query, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::query::AggregateList> {
        self.client.unary_call_async(&METHOD_QUERY_SERVICE_QUERY_ALL, req, opt)
    }

    pub fn query_all_async(&self, req: super::query::Query) -> ::grpcio::ClientUnaryReceiver<super::query::AggregateList> {
        self.query_all_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait QueryService {
    fn query_playlists(&self, ctx: ::grpcio::RpcContext, req: super::query::Query, sink: ::grpcio::UnarySink<super::playlist::PlaylistList>);
    fn query_songs(&self, ctx: ::grpcio::RpcContext, req: super::query::Query, sink: ::grpcio::UnarySink<super::song::SongList>);
    fn query_albums(&self, ctx: ::grpcio::RpcContext, req: super::query::Query, sink: ::grpcio::UnarySink<super::album::AlbumList>);
    fn query_artists(&self, ctx: ::grpcio::RpcContext, req: super::query::Query, sink: ::grpcio::UnarySink<super::artist::ArtistList>);
    fn query_all(&self, ctx: ::grpcio::RpcContext, req: super::query::Query, sink: ::grpcio::UnarySink<super::query::AggregateList>);
}

pub fn create_query_service<S: QueryService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_QUERY_SERVICE_QUERY_PLAYLISTS, move |ctx, req, resp| {
        instance.query_playlists(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_QUERY_SERVICE_QUERY_SONGS, move |ctx, req, resp| {
        instance.query_songs(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_QUERY_SERVICE_QUERY_ALBUMS, move |ctx, req, resp| {
        instance.query_albums(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_QUERY_SERVICE_QUERY_ARTISTS, move |ctx, req, resp| {
        instance.query_artists(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_QUERY_SERVICE_QUERY_ALL, move |ctx, req, resp| {
        instance.query_all(ctx, req, resp)
    });
    builder.build()
}
