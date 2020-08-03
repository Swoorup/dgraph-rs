use rand::prelude::*;
use std::sync::Mutex;

use crate::errors::DgraphError;
use crate::protos::api;
use crate::protos::api_grpc;
use crate::txn::Txn;

// Dgraph is a transaction aware client to a set of dgraph server instances.
pub struct Dgraph {
    jwt: Mutex<api::Jwt>,
    dc: Vec<api_grpc::DgraphClient>,
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

    pub fn login(&mut self, userid: String, password: String) -> Result<(), DgraphError> {
        let dc = self.any_client().expect("Cannot login. No client present");

        let login_request = api::LoginRequest {
            userid,
            password,
            ..Default::default()
        };

        let res = dc.login(&login_request)?;
        let jwt = protobuf::parse_from_bytes::<api::Jwt>(res.get_json()).unwrap();

        *self
            .jwt
            .lock()
            .expect("Unable to block or acquire lock to jwt mutex") = jwt;

        Ok(())
    }

    pub fn alter(&self, op: &api::Operation) -> Result<api::Payload, DgraphError> {
        let dc = self.any_client().expect("Cannot alter. No client present");
        let res = dc.alter(op);

        match res {
            Ok(res) => Ok(res),
            Err(err) => {
                if self.is_jwt_expired(&err) {
                    self.retry_login()?;

                    let res = dc.alter(op)?;

                    Ok(res)
                } else {
                    Err(err.into())
                }
            }
        }
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
            best_effort: false,
            client: self
                .any_client()
                .expect("Cannot create transactions. No client present!"),
            dgraph: self,
        }
    }

    pub fn new_readonly_txn(&self) -> Txn {
        let mut txn = self.new_txn();
        txn.read_only = true;
        txn
    }

    pub fn is_jwt_expired(&self, grpc_error: &grpcio::Error) -> bool {
        if let grpcio::Error::RpcFailure(rpc_failure) = grpc_error {
            if rpc_failure.status == grpcio::RpcStatusCode::UNAUTHENTICATED {
                return true;
            }
        }

        false
    }

    pub fn retry_login(&self) -> Result<(), DgraphError> {
        let mut jwt = self
            .jwt
            .lock()
            .expect("Unable to block or acquire lock to jwt mutex");

        if jwt.refresh_jwt.len() == 0 {
            return Err(DgraphError::JwtRefreshTokenEmpty);
        }

        let dc = self.any_client().expect("Cannot alter. No client present");
        let login_request = api::LoginRequest {
            refresh_token: jwt.refresh_jwt.clone(),
            ..Default::default()
        };
        let response = dc.login(&login_request)?;

        *jwt = serde_json::from_str(std::str::from_utf8(&response.json).unwrap()).unwrap();

        Ok(())
    }
}
