use std::collections::HashMap;

use crate::errors::DgraphError;
use crate::protos::api;
use crate::protos::api_grpc;

pub struct Txn<'a> {
    pub(super) context: api::TxnContext,
    pub(super) finished: bool,
    pub(super) read_only: bool,
    pub(super) best_effort: bool,
    pub(super) mutated: bool,
    pub(super) client: &'a api_grpc::DgraphClient,
    pub(super) dgraph: &'a crate::Dgraph,
}

/// Call Txn::discard() once txn goes out of scope.
/// This is safe to do so, and is possible a no-op
impl Drop for Txn<'_> {
    fn drop(&mut self) {
        let _ = self.discard();
    }
}

impl Txn<'_> {
    /// `best_effort` enables best effort in read-only queries. Using this flag
    /// will ask the Dgraph Alpha to try to get timestamps from memory in a best
    /// effort to reduce the number of outbound requests to Zero. This may yield
    /// improved latencies in read-bound datasets. Returns the transaction itself.
    pub fn best_effort(&mut self) -> Result<&Txn, DgraphError> {
        if !self.read_only {
            return Err(DgraphError::WriteTxnBestEffort);
        }
        self.best_effort = true;
        Ok(self)
    }

    pub fn query(&mut self, query: &str) -> Result<api::Response, DgraphError> {
        self.query_with_vars(query, HashMap::new())
    }

    pub fn query_with_vars(
        &mut self,
        query: &str,
        vars: HashMap<String, String>,
    ) -> Result<api::Response, DgraphError> {
        let mut request = api::Request {
            query: query.to_string(),
            vars,
            start_ts: self.context.get_start_ts(),
            read_only: self.read_only,
            best_effort: self.best_effort,
            ..Default::default()
        };

        self.do_request(&mut request)
    }

    pub fn mutate(&mut self, mu: api::Mutation) -> Result<api::Response, DgraphError> {
        let mut request = api::Request::new();
        let mutations = vec![mu.clone()];

        request.set_mutations(mutations.into());
        request.set_commit_now(mu.get_commit_now());

        self.do_request(&mut request)
    }

    pub fn do_request(&mut self, request: &mut api::Request) -> Result<api::Response, DgraphError> {
        let mutation_list = request.get_mutations();

        if self.finished {
            return Err(DgraphError::TxnFinished);
        }

        if !mutation_list.is_empty() {
            if self.read_only {
                return Err(DgraphError::TxnReadOnly);
            }

            self.mutated = true;
        }

        request.set_start_ts(self.context.get_start_ts());

        let response = match self.client.query(&request) {
            Ok(response) => response,
            Err(err) => {
                let retry_result = if self.dgraph.is_jwt_expired(&err) {
                    match self.dgraph.retry_login() {
                        Ok(_) => match self.client.query(&request) {
                            Ok(response) => Ok(response),
                            Err(err) => Err(err.into()),
                        },
                        Err(err) => Err(err),
                    }
                } else {
                    Err(err.into())
                };

                match retry_result {
                    Ok(response) => response,
                    Err(err) => {
                        let _ = self.discard();
                        return Err(err);
                    }
                }
            }
        };

        if request.commit_now {
            self.finished = true;
        }

        self.merge_context(response.get_txn())?;

        Ok(response)
    }

    pub fn commit(mut self) -> Result<(), DgraphError> {
        match (self.finished, self.read_only) {
            (true, _) => return Err(DgraphError::TxnFinished),
            (_, true) => return Err(DgraphError::TxnReadOnly),
            _ => (),
        }

        self.commit_or_abort()
    }

    pub fn discard(&mut self) -> Result<(), DgraphError> {
        self.context.aborted = true;
        self.commit_or_abort()
    }

    fn commit_or_abort(&mut self) -> Result<(), DgraphError> {
        if self.finished {
            return Ok(());
        }
        self.finished = true;

        if !self.mutated {
            return Ok(());
        }

        let res = self.client.commit_or_abort(&self.context);

        match res {
            Ok(_) => Ok(()),
            Err(err) => {
                if self.dgraph.is_jwt_expired(&err) {
                    self.dgraph.retry_login()?;
                    self.client.commit_or_abort(&self.context)?;

                    Ok(())
                } else {
                    Err(err.into())
                }
            }
        }
    }

    fn merge_context(&mut self, src: &api::TxnContext) -> Result<(), DgraphError> {
        if self.context.start_ts == 0 {
            self.context.start_ts = src.start_ts;
        }

        if self.context.start_ts != src.start_ts {
            return Err(DgraphError::StartTsMismatch);
        }

        for key in src.keys.iter() {
            self.context.keys.push(key.clone());
        }

        for pred in src.preds.iter() {
            self.context.preds.push(pred.clone());
        }

        Ok(())
    }
}
