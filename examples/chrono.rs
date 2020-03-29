use std::collections::HashMap;

use dgraph::{make_dgraph, Dgraph};
use serde_derive::{Deserialize, Serialize};

fn drop_schema(dgraph: &Dgraph) {
    let op_drop = dgraph::Operation {
        drop_all: true,
        ..Default::default()
    };

    dgraph.alter(&op_drop).expect("Failed to drop schema.");

    println!("Dropped the schema.");
}

fn set_schema(dgraph: &Dgraph) {
    let op_schema = dgraph::Operation {
        schema: r#"
            name: string @index(exact) .
            last_seen: datetime .
        "#
        .to_string(),
        ..Default::default()
    };

    dgraph.alter(&op_schema).expect("Failed to set schema.");

    println!("Altered schema.");
}

#[derive(Serialize, Deserialize, Debug)]
struct Root {
    pub people: Vec<Person>,
}

// Don't forget chrono needs to be installed with feature "serde" to be
// serializable.
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    pub uid: Option<String>,
    pub name: String,
    pub last_seen: chrono::DateTime<chrono::Utc>,
}

fn main() {
    let dgraph = make_dgraph!(dgraph::new_dgraph_client("localhost:19080"));

    println!("Connected to dgraph via gRPC at localhost:19080.");

    drop_schema(&dgraph);
    set_schema(&dgraph);

    // Insert

    let casey = Person {
        uid: None,
        name: "Casey".to_string(),
        last_seen: chrono::Utc::now(),
    };

    let mut txn = dgraph.new_txn();
    let mut mutation = dgraph::Mutation::new();

    mutation.set_set_json(serde_json::to_vec(&casey).expect("Failed to serialize JSON."));
    txn.mutate(mutation).expect("Failed to create data.");
    txn.commit().expect("Failed to commit mutation");

    // Query

    let query = r#"query all($a: string){
        people(func: eq(name, $a)) {
            uid
            name
            last_seen
        }
    }"#
    .to_string();

    let mut vars = HashMap::new();
    vars.insert("$a".to_string(), "Casey".to_string());

    let resp = dgraph
        .new_readonly_txn()
        .query_with_vars(query, vars)
        .expect("query");
    let root: Root = serde_json::from_slice(&resp.json).expect("Failed to parse JSON.");

    println!("\nQuery result for `eq(name, Casey)`:\n\n{:#?}", root);
}
