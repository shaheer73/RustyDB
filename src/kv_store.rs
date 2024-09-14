// Shared kv_store module for both server and client
pub mod kv_store {
    tonic::include_proto!("kv_store");
}
