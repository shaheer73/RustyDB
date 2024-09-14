use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::{transport::Server, Request, Response, Status};
use kv_store::kv_store_server::{KvStore, KvStoreServer};
use kv_store::{SetRequest, SetResponse, GetRequest, GetResponse, DeleteRequest, DeleteResponse};
use std::collections::HashMap;

mod kv_store {
    tonic::include_proto!("kv_store");
}

/// The server-side implementation of the RustyDB key-value store
pub struct KeyValueStore {
    store: Arc<RwLock<HashMap<String, String>>>,
}

impl KeyValueStore {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[tonic::async_trait]
impl KvStore for KeyValueStore {
    /// Handles setting a key-value pair in the store
    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        let req = request.into_inner();
        let mut store = self.store.write().await;
        store.insert(req.key, req.value);
        Ok(Response::new(SetResponse { success: true }))
    }

    /// Handles retrieving a value from the store by key
    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let req = request.into_inner();
        let store = self.store.read().await;
        if let Some(value) = store.get(&req.key) {
            Ok(Response::new(GetResponse { value: value.clone() }))
        } else {
            Err(Status::not_found("Key not found"))
        }
    }

    /// Handles deleting a key-value pair from the store
    async fn delete(&self, request: Request<DeleteRequest>) -> Result<Response<DeleteResponse>, Status> {
        let req = request.into_inner();
        let mut store = self.store.write().await;
        if store.remove(&req.key).is_some() {
            Ok(Response::new(DeleteResponse { success: true }))
        } else {
            Err(Status::not_found("Key not found"))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let kv_store = KeyValueStore::new();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(KvStoreServer::new(kv_store))
        .serve(addr)
        .await?;

    Ok(())
}
