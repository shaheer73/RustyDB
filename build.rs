fn main() {
    tonic_build::compile_protos("proto/kv_store.proto").unwrap();
}
