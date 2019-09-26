// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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

const METHOD_ECHO_SERVICE_ECHO: ::grpcio::Method<super::server::EchoRequest, super::server::EchoResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/echo.EchoService/Echo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct EchoServiceClient {
    client: ::grpcio::Client,
}

impl EchoServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        EchoServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn echo_opt(&self, req: &super::server::EchoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::server::EchoResponse> {
        self.client.unary_call(&METHOD_ECHO_SERVICE_ECHO, req, opt)
    }

    pub fn echo(&self, req: &super::server::EchoRequest) -> ::grpcio::Result<super::server::EchoResponse> {
        self.echo_opt(req, ::grpcio::CallOption::default())
    }

    pub fn echo_async_opt(&self, req: &super::server::EchoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::server::EchoResponse>> {
        self.client.unary_call_async(&METHOD_ECHO_SERVICE_ECHO, req, opt)
    }

    pub fn echo_async(&self, req: &super::server::EchoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::server::EchoResponse>> {
        self.echo_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait EchoService {
    fn echo(&mut self, ctx: ::grpcio::RpcContext, req: super::server::EchoRequest, sink: ::grpcio::UnarySink<super::server::EchoResponse>);
}

pub fn create_echo_service<S: EchoService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ECHO_SERVICE_ECHO, move |ctx, req, resp| {
        instance.echo(ctx, req, resp)
    });
    builder.build()
}
