extern crate notify_rust;
extern crate protobuf;
extern crate grpcio;
extern crate futures;

mod grpc;

use std::sync::Arc;
use notify_rust::{Notification, NotificationHint as Hint, NotificationUrgency as Urgency};
use grpc::playlist_grpc::PlaylistServiceClient;
use grpc::query_grpc::QueryServiceClient;
use grpc::song_grpc::SongServiceClient;
use grpc::query::Query;
use grpcio::{ChannelBuilder, EnvBuilder};

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = QueryServiceClient::new(ch);

    let mut req = Query::new();
    req.set_query("Sapphyre".to_owned());
    let reply = client.query_artists(req).expect("rpc");
    println!("Response received:");
    for artist in reply.get_artists() {
        println!("{} {}", artist.id, artist.title);
    }

    Notification::new()
        .summary("UpTempo")
        .body("All You See is Christ (Live)\nSapphyre Live\nSapphyre")
        .icon("/home/sumner/tmp/cover.png")
        .hint(Hint::Urgency(Urgency::Normal))
        .show()
        .unwrap();
}
