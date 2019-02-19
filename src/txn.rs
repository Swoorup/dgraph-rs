use std::collections::HashMap;
use failure::{bail, Error};

use crate::protos::api_grpc;
use crate::protos::api;

pub struct Txn<'a> {
    pub(super) context: api::TxnContext,
    pub(super) finished: bool,
    pub(super) read_only: bool,
    pub(super) mutated: bool,
    pub(super) client: &'a api_grpc::DgraphClient,
}

/// Call Txn::discard() once txn goes out of scope.
/// This is safe to do so, and is possible a no-op
impl Drop for Txn<'_> {
    fn drop(&mut self) {
        println!("Calling discard");
        let _ = self.discard();
    }
}

impl Txn<'_> {
    pub fn query(&mut self, query: impl Into<String>) -> Result<api::Response, Error> {
        self.query_with_vars(query, HashMap::new())
    }

    pub fn query_with_vars(&mut self, query: impl Into<String>, vars: HashMap<String, String>) -> Result<api::Response, Error> {
        if self.finished {
            bail!("Transaction has already been committed or discarded");
        }

        let res = self.client.query(&api::Request 
        { 
            query: query.into(), 
            vars, 
            ..Default::default()
        })?;

        let txn = match res.txn.as_ref() {
            Some(txn) => txn,
            None => bail!("Got empty Txn response back from query")
        };

        self.merge_context(txn)?;

        Ok(res)
    }

    pub fn mutate(&mut self, mut mu: api::Mutation) -> Result<api::Assigned, Error> {

        match (self.finished, self.read_only) {
            (true, _) => bail!("Txn is finished"),
            (_, true) => bail!("Txn is read only"),
            _ => ()
        }

        self.mutated = true;
        mu.start_ts = self.context.start_ts;
        let commit_now = mu.commit_now;
        let mu_res = self.client.mutate(&mu);

        let mu_res = match mu_res {
            Ok(mu_res) => mu_res,
            Err(e) => {
                let _ = self.discard();
                bail!(e);
            }
        };

        if commit_now {
            self.finished = true;
        }

        {
            let context = match mu_res.context.as_ref() {
                Some(context) => context,
                None => bail!("Missing Txn context on mutation response")
            };

            self.merge_context(context)?;
        }

        Ok(mu_res)
    }

    pub fn commit(mut self) -> Result<(), Error> {
        match (self.finished, self.read_only) {
            (true, _) => bail!("Txn is finished"),
            (_, true) => bail!("Txn is read only"),
            _ => ()
        }

        self.commit_or_abort()
    }

    pub fn discard(&mut self) -> Result<(), Error> {
        self.context.aborted = true;
        self.commit_or_abort()
    }

    fn commit_or_abort(&mut self) -> Result<(), Error> {
        if self.finished {
            return Ok(())
        }
        self.finished = true;

        if !self.mutated {
            return Ok(())
        }

        self.client.commit_or_abort(&self.context)?;

        Ok(())
    }

    fn merge_context(&mut self, src: &api::TxnContext) -> Result<(), Error> {
        if self.context.start_ts == 0 {
            self.context.start_ts = src.start_ts;
        }

        if self.context.start_ts != src.start_ts {
            bail!("self.context.start_ts != src.start_ts")
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
