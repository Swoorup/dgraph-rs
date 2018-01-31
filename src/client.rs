use std::sync::Mutex;
use failure::Error;
use rand::prelude::*;

use crate::protos::api_grpc;
use crate::protos::api;
use crate::txn::Txn;

// Dgraph is a transaction aware client to a set of dgraph server instances.
pub struct Dgraph {
    jwt: Mutex<api::Jwt>,
    dc: Vec<api_grpc::DgraphClient>
}

impl Dgraph {

    /// Creates a new Dgraph for interacting with the Dgraph store connected to in
    /// conns.
    /// The client can be backed by multiple connections (to the same server, or multiple servers in a
    /// cluster).
    ///
    /// A single client is thread safe for sharing with multiple go routines.
    pub fn new(clients: Vec<api_grpc::DgraphClient>) -> Dgraph {
        Dgraph {
            jwt: Mutex::new(api::Jwt::new()),
            dc: clients,
        }
    }

    pub fn login(&self, userid: String, password: String) -> Result<api::Response, Error> {
        let _guard = self.jwt.lock().expect("Unable to block or acquire lock to jwt mutex");
        let dc = self.any_client().expect("Cannot login. No client present");

        let login_request = api::LoginRequest {
            userid,
            password,
            ..Default::default()
        };

        let res = dc.login(&login_request)?;

        unimplemented!()
    }

    pub fn retry_login(&self, userid: String, password: String) -> Result<api::Response, Error> {
        unimplemented!()
    }

    pub fn alter(&self, op: &api::Operation) -> Result<api::Payload, Error> {
        let dc = self.any_client().expect("Cannot alter. No client present");
        let res = dc.alter(op)?;
        Ok(res)
    }

    pub fn any_client(&self) -> Option<&api_grpc::DgraphClient> {
        let mut rng = thread_rng();

        self.dc.choose(&mut rng)
    }

    pub fn new_txn(&self) -> Txn {
        Txn {
            context: Default::default(),
            finished: false,
            mutated: false,
            read_only: false,
            client: self.any_client().expect("Cannot create transactions. No client present!")
        }
    }

    pub fn new_readonly_txn(&self) -> Txn {
        let mut txn = self.new_txn();
        txn.read_only = true;
        txn
    }
}
