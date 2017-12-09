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

const METHOD_SONG_SERVICE_GET_SONG_DATA: ::grpcio::Method<super::song::SongInfo, super::song::SongChunk> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/uptempo.SongService/GetSongData",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct SongServiceClient {
    client: ::grpcio::Client,
}

impl SongServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        SongServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_song_data_opt(&self, req: super::song::SongInfo, opt: ::grpcio::CallOption) -> ::grpcio::ClientSStreamReceiver<super::song::SongChunk> {
        self.client.server_streaming(&METHOD_SONG_SERVICE_GET_SONG_DATA, req, opt)
    }

    pub fn get_song_data(&self, req: super::song::SongInfo) -> ::grpcio::ClientSStreamReceiver<super::song::SongChunk> {
        self.get_song_data_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait SongService {
    fn get_song_data(&self, ctx: ::grpcio::RpcContext, req: super::song::SongInfo, sink: ::grpcio::ServerStreamingSink<super::song::SongChunk>);
}

pub fn create_song_service<S: SongService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_SONG_SERVICE_GET_SONG_DATA, move |ctx, req, resp| {
        instance.get_song_data(ctx, req, resp)
    });
    builder.build()
}
