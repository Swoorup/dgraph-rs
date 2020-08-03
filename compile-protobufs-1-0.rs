extern crate protoc_grpcio;

fn main() {
    let proto_root = "src/protos/dgraph10";
    println!("cargo:rerun-if-changed={}", proto_root);

    protoc_grpcio::compile_grpc_protos(
        &["api.proto"],
        &[proto_root],
        &proto_root,
        Some(protobuf_codegen::Customize {
            serde_derive: Some(true),
            ..Default::default()
        }),
    )
    .expect("Failed to compile gRPC definitions!");
}
