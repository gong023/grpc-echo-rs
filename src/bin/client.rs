#[macro_use]
extern crate log;

#[path = "../log_utils.rs"]
mod log_util;

use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};
use grpc_echo_rs::server::EchoRequest;
use grpc_echo_rs::server_grpc::EchoClient;

fn main() {
    let _guard = log_util::init_log(None);
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = EchoClient::new(ch);

    let mut req = EchoRequest::default();
    req.set_message("hello world!".to_owned());
    let resp = client.echo(&req).expect("rpc");
    info!("response: {}", resp.get_message());
}
