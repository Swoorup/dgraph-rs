use std::collections::HashMap;

use dgraph::make_dgraph;
use serde_derive::{Deserialize, Serialize};
use serde_json;

mod common;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UidJson {
    pub uids: Vec<Uid>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Uid {
    pub uid: String,
}

#[test]
fn it_runs_simple_query() {
    let dgraph = make_dgraph!(dgraph::new_dgraph_client(common::DGRAPH_URL));

    let uid = "0x1";
    let query = format!(
        r#"{{
        uids(func: uid({})) {{
            uid,
        }}
    }}"#,
        uid
    )
    .to_string();
    let resp = dgraph.new_readonly_txn().query(query);
    let json: UidJson = serde_json::from_slice(&resp.unwrap().json).unwrap();

    assert_eq!(json.uids[0].uid, uid);
}

#[test]
fn it_runs_query_with_vars() {
    let dgraph = make_dgraph!(dgraph::new_dgraph_client(common::DGRAPH_URL));

    let uid = "0x1";
    let query = r#"query all($a: string){
        uids(func: uid($a)) {
            uid,
        }
    }"#
    .to_string();
    let mut vars = HashMap::new();
    vars.insert("$a".to_string(), uid.to_string());
    let resp = dgraph.new_readonly_txn().query_with_vars(query, vars);
    let json: UidJson = serde_json::from_slice(&resp.unwrap().json).unwrap();

    assert_eq!(json.uids[0].uid, uid);
}

#[test]
fn it_returns_error_if_mandatory_var_is_omitted() {
    let dgraph = make_dgraph!(dgraph::new_dgraph_client(common::DGRAPH_URL));

    let query = r#"query all($a: string!){
        uids(func: eq(name, $a)) {
            uid
        }
    }"#
    .to_string();
    let vars = HashMap::new();
    let resp = dgraph.new_readonly_txn().query_with_vars(query, vars);

    assert_eq!(resp.is_err(), true);
}

#[test]
fn it_commits_a_mutation() {
    let dgraph = make_dgraph!(dgraph::new_dgraph_client(common::DGRAPH_URL));

    let mut txn = dgraph.new_txn();
    let mut mutation = dgraph::Mutation::new();

    mutation.set_set_json(br#"{"name": "Alice"}"#.to_vec());
    txn.mutate(mutation).unwrap();
    let result = txn.commit();

    assert_eq!(result.is_ok(), true);
}

#[test]
fn it_returns_error_if_autocommited_mutation_is_commited_again() {
    let dgraph = make_dgraph!(dgraph::new_dgraph_client(common::DGRAPH_URL));

    let mut txn = dgraph.new_txn();
    let mut mutation = dgraph::Mutation {
        commit_now: true,
        ..Default::default()
    };

    mutation.set_set_json(br#"{"name": "Alice"}"#.to_vec());
    txn.mutate(mutation).unwrap();
    let result = txn.commit();

    assert_eq!(result.is_err(), true);
}

#[test]
fn it_does_not_allow_mutation_in_readonly_transaction() {
    let dgraph = make_dgraph!(dgraph::new_dgraph_client(common::DGRAPH_URL));

    let mut txn = dgraph.new_readonly_txn();
    let mut mutation = dgraph::Mutation::new();

    mutation.set_set_json(br#"{"name": "Alice"}"#.to_vec());
    let result = txn.mutate(mutation);

    assert_eq!(result.is_err(), true);
}

// #[test]
// fn it_discards_a_transaction() {
//     let dgraph = make_dgraph!(dgraph::new_dgraph_client(common::DGRAPH_URL));

//     let mut txn = dgraph.new_txn();
//     let mut mutation = dgraph::Mutation::new();

//     mutation.set_set_json(br#"{"name": "Alice"}"#.to_vec());
//     txn.mutate(mutation).unwrap();
//     let result = txn.discard();

//     assert_eq!(result.is_ok(), true);
// }

#[test]
fn it_does_nothing_if_autocommited_mutation_is_discarded() {
    let dgraph = make_dgraph!(dgraph::new_dgraph_client(common::DGRAPH_URL));

    let mut txn = dgraph.new_txn();
    let mut mutation = dgraph::Mutation {
        commit_now: true,
        ..Default::default()
    };

    mutation.set_set_json(br#"{"name": "Alice"}"#.to_vec());
    txn.mutate(mutation).unwrap();
    let result = txn.discard();

    assert_eq!(result.is_ok(), true);
}

#[test]
fn it_does_not_commit_discarded_transaction() {
    let dgraph = make_dgraph!(dgraph::new_dgraph_client(common::DGRAPH_URL));

    let mut txn = dgraph.new_txn();
    let mut mutation = dgraph::Mutation::new();

    mutation.set_set_json(br#"{"name": "Alice"}"#.to_vec());
    txn.mutate(mutation).unwrap();
    let _ = txn.discard();
    let result = txn.commit();

    assert_eq!(result.is_err(), true);
}
