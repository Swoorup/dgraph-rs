mod client;
mod errors;
mod protos;
mod txn;

use grpcio::{ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder};
use std::sync::Arc;

pub use grpcio;

pub use client::Dgraph;
pub use errors::DgraphError;
pub use protos::api::*;
pub use protos::api_grpc::*;
pub use txn::Txn;

#[cfg(feature = "with-serde")]
extern crate serde;
#[cfg(feature = "with-serde")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "with-serde")]
extern crate serde_json;

pub fn new_secure_dgraph_client(
    addr: &str,
    root_ca: Vec<u8>,
    cert: Vec<u8>,
    private_key: Vec<u8>,
) -> DgraphClient {
    let env = Arc::new(EnvBuilder::new().build());
    let credentials = ChannelCredentialsBuilder::new()
        .root_cert(root_ca)
        .cert(cert, private_key)
        .build();
    let channel = ChannelBuilder::new(env).secure_connect(addr, credentials);
    DgraphClient::new(channel)
}

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
