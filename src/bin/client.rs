// Copyright 2019 gong023
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// copied from https://github.com/pingcap/grpc-rs/blob/master/tests-and-examples/examples/hello_world/client.rs.
// changed to comply with compiling.

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
