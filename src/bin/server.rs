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
// copied from https://github.com/pingcap/grpc-rs/blob/86c2bc1aec0757063de427b0ace79226489ffd78/tests-and-examples/examples/hello_world/server.rs
// changed to comply with compiling.

#[macro_use]
extern crate log;

#[path = "../log_utils.rs"]
mod log_util;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{ChannelBuilder, Environment, ServerBuilder, UnarySink};

use grpc_echo_rs::server::{EchoRequest, EchoResponse};
use grpc_echo_rs::server_grpc::{create_echo, Echo};

#[derive(Clone)]
struct EchoService;

impl Echo for EchoService {
    fn echo(&mut self, ctx: ::grpcio::RpcContext, req: EchoRequest, sink: UnarySink<EchoResponse>) {
        let mut resp = EchoResponse::default();
        resp.set_message(req.get_message().into());
        let f = sink.success(resp).map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
}

fn main() {
    let _guard = log_util::init_log(None);
    let env = Arc::new(Environment::new(1));
    let service = create_echo(EchoService);

    let ch_builder = ChannelBuilder::new(env.clone());

    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50_051)
        .channel_args(ch_builder.build_args())
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        info!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        info!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
