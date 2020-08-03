use dgraph::{make_dgraph, Dgraph, Operation};

mod common;

fn is_connected(dgraph: &Dgraph) -> bool {
    let q = "schema {}".to_string();
    let response = dgraph.new_readonly_txn().query(&q);

    response.is_ok()
}

#[test]
fn it_connects() {
    let dgraph = make_dgraph!(dgraph::new_dgraph_client(common::DGRAPH_URL));

    assert_eq!(is_connected(&dgraph), true);
}

#[test]
fn it_does_not_connect_to_wrong_url() {
    let dgraph = make_dgraph!(dgraph::new_dgraph_client("no_dgraph_url:9080"));

    assert_eq!(is_connected(&dgraph), false);
}

#[test]
fn it_alters_schema() {
    let dgraph = make_dgraph!(dgraph::new_dgraph_client(common::DGRAPH_URL));

    let result = dgraph.alter(&Operation {
        schema: "something: string .".to_string(),
        ..Default::default()
    });

    assert_eq!(result.is_ok(), true);
}

#[test]
#[should_panic]
fn it_does_not_alter_without_client() {
    let dgraph = make_dgraph!();
    let _ = dgraph.alter(&Operation {
        schema: "something: string .".to_string(),
        ..Default::default()
    });
}

#[test]
#[should_panic]
fn it_does_not_crate_transaction_without_client() {
    let dgraph = make_dgraph!();
    dgraph.new_txn();
}
