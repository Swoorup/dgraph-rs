cfg_if::cfg_if! {
    if #[cfg(feature = "dgraph-1-0")] {
        pub mod dgraph10;

        pub use dgraph10::api;
        pub use dgraph10::api_grpc;
    } else if #[cfg(feature = "dgraph-1-1")] {
        pub mod dgraph11;

        pub use dgraph11::api;
        pub use dgraph11::api_grpc;
    }
}
