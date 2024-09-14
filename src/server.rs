use tonic::{transport::Server, Request, Response, Status};
use kv_store::kv_store_server::{KvStore, KvStoreServer};
use kv_store::{SetRequest, SetResponse, GetRequest, GetResponse, DeleteRequest, DeleteResponse};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

mod kv_store {
    tonic::include_proto!("kv_store");
}

/// Define the hashing function to distribute keys across shards
fn get_shard(key: &str, num_shards: usize) -> usize {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hash::hash(&key, &mut hasher);
    (hasher.finish() as usize) % num_shards
}

/// Define the Sharded Key-Value Store
pub struct ShardedKeyValueStore {
    shards: Vec<Arc<RwLock<HashMap<String, String>>>>,
    num_shards: usize,
}

impl ShardedKeyValueStore {
    pub fn new(num_shards: usize) -> Self {
        let mut shards = Vec::new();
        for _ in 0..num_shards {
            shards.push(Arc::new(RwLock::new(HashMap::new())));
        }
        Self { shards, num_shards }
    }

    /// Get the shard for the given key
    fn get_shard(&self, key: &str) -> Arc<RwLock<HashMap<String, String>>> {
        let shard_index = get_shard(key, self.num_shards);
        Arc::clone(&self.shards[shard_index])
    }

    /// Set a key-value pair
    pub async fn set(&self, key: String, value: String) {
        let shard = self.get_shard(&key);
        let mut shard = shard.write().await;
        shard.insert(key, value);
    }

    /// Get a value for the given key
    pub async fn get(&self, key: String) -> Option<String> {
        let shard = self.get_shard(&key);
        let shard = shard.read().await;
        shard.get(&key).cloned()
    }

    /// Delete a key-value pair
    pub async fn delete(&self, key: String) {
        let shard = self.get_shard(&key);
        let mut shard = shard.write().await;
        shard.remove(&key);
    }
}

#[tonic::async_trait]
impl KvStore for ShardedKeyValueStore {
    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        let req = request.into_inner();
        self.set(req.key, req.value).await;
        Ok(Response::new(SetResponse { success: true }))
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let req = request.into_inner();
        if let Some(value) = self.get(req.key).await {
            Ok(Response::new(GetResponse { value }))
        } else {
            Err(Status::not_found("Key not found"))
        }
    }

    async fn delete(&self, request: Request<DeleteRequest>) -> Result<Response<DeleteResponse>, Status> {
        let req = request.into_inner();
        self.delete(req.key).await;
        Ok(Response::new(DeleteResponse { success: true }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::]:50051".parse()?;
    let kv_store = ShardedKeyValueStore::new(3); // Use 3 shards

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(KvStoreServer::new(kv_store))
        .serve(addr)
        .await?;

    Ok(())
}
