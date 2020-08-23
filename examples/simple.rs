use std::collections::HashMap;

use chrono::prelude::*;
use dgraph::{make_dgraph, Dgraph};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
struct Root {
    me: Vec<Person>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct School {
    name: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct Location {
    #[serde(rename = "type")]
    kind: String,
    coordinates: Vec<f64>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct Person {
    name: String,
    age: Option<u8>,
    dob: Option<DateTime<Utc>>,
    married: Option<bool>,
    friend: Option<Vec<Person>>,
    loc: Option<Location>,
    school: Option<Vec<School>>,
}

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
            age: int .
            married: bool .
            loc: geo .
            dob: datetime .
        "#
        .to_string(),
        ..Default::default()
    };

    dgraph.alter(&op_schema).expect("Failed to set schema.");

    println!("Altered schema.");
}

fn main() {
    let dgraph = make_dgraph!(dgraph::new_dgraph_client("localhost:19080"));

    println!("Connected to dgraph via gRPC at localhost:19080.");

    drop_schema(&dgraph);
    set_schema(&dgraph);

    let mut txn = dgraph.new_txn();

    // While setting an object if a struct has a Uid then its properties in
    // the graph are updated. Else a new node is created.
    //
    // In the example below new nodes for Alice, Bob and Charlie and school
    // are created (since they don't have a Uid).
    let date_of_birth = Utc.ymd(1980, 1, 1).and_hms(23, 0, 0);
    let p = Person {
        name: "Alice".to_string(),
        age: Some(26),
        married: Some(true),
        loc: Some(Location {
            kind: "Point".to_string(),
            coordinates: vec![1.1f64, 2f64],
        }),
        dob: Some(date_of_birth),
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
    mutation.set_set_json(serde_json::to_vec(&p).expect("Failed to serialize JSON."));
    let assigned = txn.mutate(mutation).expect("Failed to create data.");

    // Commit transaction
    txn.commit().expect("Failed to commit mutation");

    println!("All created nodes (map from blank node names to uids):\n");

    for (key, val) in assigned.uids.iter() {
        println!("\t{} => {}", key, val);
    }

    let query = r#"query all($a: string){
        me(func: eq(name, $a)) {
            uid
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
    }"#;

    let mut vars = HashMap::new();
    vars.insert("$a".to_string(), "Alice".to_string());

    let resp = dgraph
        .new_readonly_txn()
        .query_with_vars(&query, vars)
        .expect("query");
    let root: Root = serde_json::from_slice(&resp.json).expect("Failed to convert slice to JSON.");

    println!("\nQuery result for `eq(name, Alice)`:\n\n{:#?}", root);
}
