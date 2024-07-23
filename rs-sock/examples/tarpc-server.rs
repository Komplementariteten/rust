use std::future::Future;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::time::Duration;

use futures::{future, prelude::*};
use rand::{distributions::{Distribution, Uniform}, thread_rng};
use tarpc::context::Context;
use tarpc::server;
use tarpc::server::Channel;
use tarpc::server::incoming::Incoming;
use tarpc::tokio_serde::formats::Bincode;
use tokio::time;

use crate::tarpc_shared::World;

pub mod tarpc_shared;

#[derive(Clone)]
struct HelloServer(SocketAddr);

impl World for HelloServer {
    async fn hello(self, _: Context, name: String) -> String {
        let sleep_time =
            Duration::from_millis(Uniform::new_inclusive(1, 10).sample(&mut thread_rng()));
        time::sleep(sleep_time).await;
        format!("Hello, {name}! You are connected from {}", self.0)
    }
}

async fn spawn(fut: impl Future<Output=()> + Send + 'static) {
    tokio::spawn(fut);
}

#[tokio::main]
async fn main() -> () {
    let s_addr = (IpAddr::V6(Ipv6Addr::LOCALHOST), 12001);
    let mut listener = tarpc::serde_transport::tcp::listen(&s_addr, Bincode::default).await.expect("Failed to establish listener");
    listener.config_mut().max_frame_length(usize::MAX);
    let _ = listener
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .max_channels_per_key(1, |t| t.transport().peer_addr().unwrap().ip())
        .map(|channel| {
            let server = HelloServer(channel.transport().peer_addr().unwrap());
            return channel.execute(server.serve()).for_each(spawn);
        }).buffer_unordered(10)
        .for_each(|_| async {}).await;
    ()
}