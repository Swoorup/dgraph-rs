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

const METHOD_DGRAPH_LOGIN: ::grpcio::Method<super::api::LoginRequest, super::api::Response> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/api.Dgraph/Login",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DGRAPH_QUERY: ::grpcio::Method<super::api::Request, super::api::Response> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/api.Dgraph/Query",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DGRAPH_MUTATE: ::grpcio::Method<super::api::Mutation, super::api::Assigned> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/api.Dgraph/Mutate",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DGRAPH_ALTER: ::grpcio::Method<super::api::Operation, super::api::Payload> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/api.Dgraph/Alter",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DGRAPH_COMMIT_OR_ABORT: ::grpcio::Method<super::api::TxnContext, super::api::TxnContext> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/api.Dgraph/CommitOrAbort",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DGRAPH_CHECK_VERSION: ::grpcio::Method<super::api::Check, super::api::Version> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/api.Dgraph/CheckVersion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct DgraphClient {
    client: ::grpcio::Client,
}

impl DgraphClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        DgraphClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn login_opt(&self, req: &super::api::LoginRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::api::Response> {
        self.client.unary_call(&METHOD_DGRAPH_LOGIN, req, opt)
    }

    pub fn login(&self, req: &super::api::LoginRequest) -> ::grpcio::Result<super::api::Response> {
        self.login_opt(req, ::grpcio::CallOption::default())
    }

    pub fn login_async_opt(&self, req: &super::api::LoginRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::api::Response>> {
        self.client.unary_call_async(&METHOD_DGRAPH_LOGIN, req, opt)
    }

    pub fn login_async(&self, req: &super::api::LoginRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::api::Response>> {
        self.login_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_opt(&self, req: &super::api::Request, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::api::Response> {
        self.client.unary_call(&METHOD_DGRAPH_QUERY, req, opt)
    }

    pub fn query(&self, req: &super::api::Request) -> ::grpcio::Result<super::api::Response> {
        self.query_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_async_opt(&self, req: &super::api::Request, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::api::Response>> {
        self.client.unary_call_async(&METHOD_DGRAPH_QUERY, req, opt)
    }

    pub fn query_async(&self, req: &super::api::Request) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::api::Response>> {
        self.query_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn mutate_opt(&self, req: &super::api::Mutation, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::api::Assigned> {
        self.client.unary_call(&METHOD_DGRAPH_MUTATE, req, opt)
    }

    pub fn mutate(&self, req: &super::api::Mutation) -> ::grpcio::Result<super::api::Assigned> {
        self.mutate_opt(req, ::grpcio::CallOption::default())
    }

    pub fn mutate_async_opt(&self, req: &super::api::Mutation, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::api::Assigned>> {
        self.client.unary_call_async(&METHOD_DGRAPH_MUTATE, req, opt)
    }

    pub fn mutate_async(&self, req: &super::api::Mutation) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::api::Assigned>> {
        self.mutate_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn alter_opt(&self, req: &super::api::Operation, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::api::Payload> {
        self.client.unary_call(&METHOD_DGRAPH_ALTER, req, opt)
    }

    pub fn alter(&self, req: &super::api::Operation) -> ::grpcio::Result<super::api::Payload> {
        self.alter_opt(req, ::grpcio::CallOption::default())
    }

    pub fn alter_async_opt(&self, req: &super::api::Operation, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::api::Payload>> {
        self.client.unary_call_async(&METHOD_DGRAPH_ALTER, req, opt)
    }

    pub fn alter_async(&self, req: &super::api::Operation) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::api::Payload>> {
        self.alter_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn commit_or_abort_opt(&self, req: &super::api::TxnContext, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::api::TxnContext> {
        self.client.unary_call(&METHOD_DGRAPH_COMMIT_OR_ABORT, req, opt)
    }

    pub fn commit_or_abort(&self, req: &super::api::TxnContext) -> ::grpcio::Result<super::api::TxnContext> {
        self.commit_or_abort_opt(req, ::grpcio::CallOption::default())
    }

    pub fn commit_or_abort_async_opt(&self, req: &super::api::TxnContext, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::api::TxnContext>> {
        self.client.unary_call_async(&METHOD_DGRAPH_COMMIT_OR_ABORT, req, opt)
    }

    pub fn commit_or_abort_async(&self, req: &super::api::TxnContext) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::api::TxnContext>> {
        self.commit_or_abort_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn check_version_opt(&self, req: &super::api::Check, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::api::Version> {
        self.client.unary_call(&METHOD_DGRAPH_CHECK_VERSION, req, opt)
    }

    pub fn check_version(&self, req: &super::api::Check) -> ::grpcio::Result<super::api::Version> {
        self.check_version_opt(req, ::grpcio::CallOption::default())
    }

    pub fn check_version_async_opt(&self, req: &super::api::Check, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::api::Version>> {
        self.client.unary_call_async(&METHOD_DGRAPH_CHECK_VERSION, req, opt)
    }

    pub fn check_version_async(&self, req: &super::api::Check) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::api::Version>> {
        self.check_version_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Dgraph {
    fn login(&mut self, ctx: ::grpcio::RpcContext, req: super::api::LoginRequest, sink: ::grpcio::UnarySink<super::api::Response>);
    fn query(&mut self, ctx: ::grpcio::RpcContext, req: super::api::Request, sink: ::grpcio::UnarySink<super::api::Response>);
    fn mutate(&mut self, ctx: ::grpcio::RpcContext, req: super::api::Mutation, sink: ::grpcio::UnarySink<super::api::Assigned>);
    fn alter(&mut self, ctx: ::grpcio::RpcContext, req: super::api::Operation, sink: ::grpcio::UnarySink<super::api::Payload>);
    fn commit_or_abort(&mut self, ctx: ::grpcio::RpcContext, req: super::api::TxnContext, sink: ::grpcio::UnarySink<super::api::TxnContext>);
    fn check_version(&mut self, ctx: ::grpcio::RpcContext, req: super::api::Check, sink: ::grpcio::UnarySink<super::api::Version>);
}

pub fn create_dgraph<S: Dgraph + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DGRAPH_LOGIN, move |ctx, req, resp| {
        instance.login(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DGRAPH_QUERY, move |ctx, req, resp| {
        instance.query(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DGRAPH_MUTATE, move |ctx, req, resp| {
        instance.mutate(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DGRAPH_ALTER, move |ctx, req, resp| {
        instance.alter(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DGRAPH_COMMIT_OR_ABORT, move |ctx, req, resp| {
        instance.commit_or_abort(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DGRAPH_CHECK_VERSION, move |ctx, req, resp| {
        instance.check_version(ctx, req, resp)
    });
    builder.build()
}
