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

const METHOD_PLAYLIST_SERVICE_GET_PLAYLIST: ::grpcio::Method<super::playlist::PlaylistInfo, super::playlist::Playlist> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/uptempo.PlaylistService/GetPlaylist",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct PlaylistServiceClient {
    client: ::grpcio::Client,
}

impl PlaylistServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        PlaylistServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_playlist_opt(&self, req: super::playlist::PlaylistInfo, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::playlist::Playlist> {
        self.client.unary_call(&METHOD_PLAYLIST_SERVICE_GET_PLAYLIST, req, opt)
    }

    pub fn get_playlist(&self, req: super::playlist::PlaylistInfo) -> ::grpcio::Result<super::playlist::Playlist> {
        self.get_playlist_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_playlist_async_opt(&self, req: super::playlist::PlaylistInfo, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::playlist::Playlist> {
        self.client.unary_call_async(&METHOD_PLAYLIST_SERVICE_GET_PLAYLIST, req, opt)
    }

    pub fn get_playlist_async(&self, req: super::playlist::PlaylistInfo) -> ::grpcio::ClientUnaryReceiver<super::playlist::Playlist> {
        self.get_playlist_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait PlaylistService {
    fn get_playlist(&self, ctx: ::grpcio::RpcContext, req: super::playlist::PlaylistInfo, sink: ::grpcio::UnarySink<super::playlist::Playlist>);
}

pub fn create_playlist_service<S: PlaylistService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PLAYLIST_SERVICE_GET_PLAYLIST, move |ctx, req, resp| {
        instance.get_playlist(ctx, req, resp)
    });
    builder.build()
}
