use std::collections::HashMap;

use chrono::prelude::*;
use dgraph::{make_dgraph, Dgraph};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Root {
    pub me: Vec<Person>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct School {
    pub name: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Location {
    #[serde(rename = "type")]
    pub kind: String,
    pub coordinates: Vec<f64>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Person {
    pub name: String,
    pub age: Option<u8>,
    pub dob: Option<DateTime<Utc>>,
    pub married: Option<bool>,
    pub friend: Option<Vec<Person>>,
    pub loc: Option<Location>,
    pub school: Option<Vec<School>>,
}

fn drop_all(dgraph: &Dgraph) {
    let op_cleanup = dgraph::Operation {
        drop_all: true,
        ..Default::default()
    };

    dgraph.alter(&op_cleanup).expect("drop schema");
}

fn set_schema(dgraph: &Dgraph) {
    let op_schema = dgraph::Operation {
        schema: r#"
            name: string @index(exact) .
            age: int .
            married: bool .
            loc: geo .
            dob: datetime .
        "#
        .to_string(),
        ..Default::default()
    };

    dgraph.alter(&op_schema).expect("set schema");
}

fn create_data(dgraph: &Dgraph) {
    let mut txn = dgraph.new_txn();

    let dob = Utc.ymd(1980, 1, 1).and_hms(23, 0, 0);
    // While setting an object if a struct has a Uid then its properties in the graph are updated
    // else a new node is created.
    // In the example below new nodes for Alice, Bob and Charlie and school are created (since they
    // dont have a Uid).
    let p = Person {
        name: "Alice".to_string(),
        age: Some(26),
        married: Some(true),
        loc: Some(Location {
            kind: "Point".to_string(),
            coordinates: vec![1.1f64, 2f64],
        }),
        dob: Some(dob),
        friend: Some(vec![
            Person {
                name: "Bob".to_string(),
                age: Some(24),
                ..Default::default()
            },
            Person {
                name: "Charlie".to_string(),
                age: Some(29),
                ..Default::default()
            },
        ]),
        school: Some(vec![School {
            name: "Crown Public School".to_string(),
        }]),
    };

    // Run mutation
    let mut mutation = dgraph::Mutation::new();
    mutation.set_set_json(serde_json::to_vec(&p).expect("invalid json"));
    let assigned = txn.mutate(mutation).expect("failed to create data");

    // Commit transaction
    txn.commit().expect("Fail to commit mutation");

    // Get uid of the outermost object (person named "Alice").
    // Assigned#getUidsMap() returns a map from blank node names to uids.
    // For a json mutation, blank node names "blank-0", "blank-1", ... are used
    // for all the created nodes.
    println!(
        "Created person named 'Alice' with uid = {}",
        assigned.uids["blank-0"]
    );

    println!("All created nodes (map from blank node names to uids):");
    for (key, val) in assigned.uids.iter() {
        println!("\t{} => {}", key, val);
    }
}

fn query_data(dgraph: &Dgraph) {
    let query = r#"query all($a: string){
        me(func: eq(name, $a)) {
            name
            age
            married
            loc
            dob
            friend {
                name
                age
            }
            school {
                name
            }
        }
    }"#
    .to_string();

    let mut vars = HashMap::new();
    vars.insert("$a".to_string(), "Alice".to_string());

    let resp = dgraph
        .new_readonly_txn()
        .query_with_vars(query, vars)
        .expect("query");
    let root: Root = serde_json::from_slice(&resp.json).expect("parsing");
    println!("Root: {:#?}", root);
}

fn main() {
    println!("connect to dgraph via grpc at localhost:9080");

    let dgraph = make_dgraph!(dgraph::new_dgraph_client("localhost:19080"));

    println!("dropping all schema");
    drop_all(&dgraph);

    println!("setup schema");
    set_schema(&dgraph);

    println!("push data");
    create_data(&dgraph);

    println!("query");
    query_data(&dgraph);
}
