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
            location: geo @index(geo) .
        "#
        .to_string(),
        ..Default::default()
    };

    dgraph.alter(&op_schema).expect("Failed to set schema.");

    println!("Altered schema.");
}

#[derive(Serialize, Deserialize, Debug)]
struct Root {
    pub cities: Vec<City>,
}

#[derive(Serialize, Deserialize, Debug)]
struct City {
    pub uid: Option<String>,
    pub name: String,
    pub location: geojson::Geometry,
}

fn main() {
    let dgraph = make_dgraph!(dgraph::new_dgraph_client("localhost:19080"));

    println!("Connected to dgraph via gRPC at localhost:19080.");

    drop_schema(&dgraph);
    set_schema(&dgraph);

    // Insert

    let city = City {
        uid: None,
        name: "Bratislava".to_string(),
        location: geojson::Geometry::new(geojson::Value::Point(vec![48.148_16, 17.106_74])),
    };

    let mut txn = dgraph.new_txn();
    let mut mutation = dgraph::Mutation::new();

    mutation.set_set_json(serde_json::to_vec(&city).expect("Failed to serialize JSON."));
    txn.mutate(mutation).expect("Failed to create data.");
    txn.commit().expect("Failed to commit mutation");

    // Query

    let query = r#"query all($a: string){
        cities(func: eq(name, $a)) {
            uid
            name
            location
        }
    }"#
    .to_string();

    let mut vars = HashMap::new();
    vars.insert("$a".to_string(), "Bratislava".to_string());

    let resp = dgraph
        .new_readonly_txn()
        .query_with_vars(&query, vars)
        .expect("query");
    let root: Root = serde_json::from_slice(&resp.json).expect("Failed to parse JSON.");

    println!("\nQuery result for `eq(name, Bratislava)`:\n\n{:#?}", root);
}
