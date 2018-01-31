#![allow(unused_variables)]


mod protos;
mod txn;
mod client;

use grpcio::{ChannelBuilder, EnvBuilder};
use std::sync::Arc;

pub use protos::api_grpc::*;
pub use protos::api::*;
pub use txn::Txn;
pub use client::Dgraph;

pub fn new_dgraph_client(addr: &str) -> DgraphClient {
    let env = Arc::new(EnvBuilder::new().build());
    let channel = ChannelBuilder::new(env).connect(addr);
    DgraphClient::new(channel)
}

#[macro_export]
macro_rules! make_dgraph {
    ($( $x:expr ),* ) => {
        {
            let mut temp_vec: Vec<dgraph::DgraphClient> = vec![$($x,)*];
            dgraph::Dgraph::new(temp_vec)
        }
    };
}