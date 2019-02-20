A rust client for dgraph
========================

Dgraph Rust client which communicates with the server using
[gRPC](https://grpc.io/).

Before using this client, it is highly recommended to go through
[tour.dgraph.io] and [docs.dgraph.io] to understand how to run and work
with Dgraph.

[docs.dgraph.io]:https://docs.dgraph.io
[tour.dgraph.io]:https://tour.dgraph.io


## Table of contents

- [Install](#install)
- [Using a client](#using-a-client)
  - [Create a client](#create-a-client)
  - [Alter the database](#alter-the-database)
  - [Create a transaction](#create-a-transaction)
  - [Run a mutation](#run-a-mutation)
  - [Run a query](#run-a-query)
  - [Commit a transaction](#commit-a-transaction)

## Install

Dgraph-rs is available on crates.io. Add the following dependency to your
Cargo.toml

```toml
[dependencies]
dgraph = "0.1.1"
```

## Using a client

### Create a client

`Dgraph` object can be initialised by passing it a list of `dgraph::DgraphClient`
clients as a vector. Connecting to multiple Dgraph servers in the same
cluster allows for better distribution of workload. The library provides
a macro to do so.

The following code snippet shows just one connection.

```rust
  let dgraph = make_dgraph!(dgraph::new_dgraph_client("localhost:9080"));
```

### Alter the database

To set the schema, create an instance of `dgraph::Operation` and use the
`Alter` endpoint.

```rust
let op = dgraph::Operation{
  Schema: "name: string @index(exact) .", ..Default::default()
};
let result = dgraph.alter(&op);
// Check error
```

`Operation` contains other fields as well, including `DropAttr` and `DropAll`.
`DropAll` is useful if you wish to discard all the data, and start from a clean
slate, without bringing the instance down. `DropAttr` is used to drop all the data
related to a predicate.

### Create a transaction

To create a transaction, call `dgraph.new_txn()`, which returns a `dgraph::Txn` object. This
operation incurs no network overhead.

Once `dgraph::Txn` goes out of scope, `txn.discard()` is automatically called via the `Drop` trait.
Calling `txn.discard()` after `txn.commit()` is a no-op and calling this multiple
times has no additional side-effects.

```rust
let txn = dgraph.new_txn();
```

### Run a mutation

`txn.mutate(mu)` runs a mutation. It takes in a `dgraph::Mutation`
object. You can set the data using JSON or RDF N-Quad format.

We define a Person struct to represent a Person and marshal an instance of it to use with `Mutation`
object.

```rust
#[derive(Serialize, Deserialize, Default, Debug)]
struct Person {
  uid: String,
  name: String,
}

let p = Person {
  uid:  "_:alice".to_string(),
  Name: "Alice".to_string(),
}

let pb = serde_json::to_vec(&p).expect("Invalid json");

let mut mu = dgraph::Mutation {
  json: pb, ..Default::default()
};

let assigned = txn.mutate(mu).expect("failed to create data");
```

For a more complete example, see the simple example [simple](https://github.com/Swoorup/dgraph-rs/blob/master/examples/simple/main.rs).

Sometimes, you only want to commit a mutation, without querying anything further.
In such cases, you can use `mu.commit_now = true` to indicate that the
mutation must be immediately committed.

### Run a query

You can run a query by calling `txn.query(q)`. You will need to pass in a GraphQL+- query string. If
you want to pass an additional map of any variables that you might want to set in the query, call
`txn.query_with_vars(q, vars)` with the variables map as third argument.

Let's run the following query with a variable $a:

```rust
let q = r#"query all($a: string) {
    all(func: eq(name, $a)) {
      name
    }
  }"#.to_string();

let mut vars = HashMap::new();
vars.insert("$a".to_string(), "Alice".to_string());

let resp = dgraph.new_readonly_txn().query_with_vars(q, vars).expect("query");
let root: Root = serde_json::from_slice(&resp.json).expect("parsing");
println!("Root: {:#?}", root);
```

When running a schema query, the schema response is found in the `Schema` field of `dgraph::Response`.

```rust
let q = r#"schema(pred: [name]) {
  type
  index
  reverse
  tokenizer
  list
  count
  upsert
  lang
}"#.to_string();

let resp = txn.query(q)?;
println!("{:#?}", resp.schema);
```

### Commit a transaction

A transaction can be committed using the `txn.commit()` method. If your transaction
consisted solely of calls to `txn.query` or `txn.query_with_vars`, and no calls to
`txn.mutate`, then calling `txn.commit` is not necessary.

An error will be returned if other transactions running concurrently modify the same
data that was modified in this transaction. It is up to the user to retry
transactions when they fail.

```rust
let txn = dgraph.new_txn();
// Perform some queries and mutations.

let res = txn.commit();
if res.is_err() {
  // Retry or handle error
}
```

### Contribution

Contribution are welcomed. Feel free to raise an issue, for feature requests, bug fixes and improvements.
