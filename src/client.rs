use kv_store::kv_store_client::KvStoreClient;
use kv_store::{SetRequest, GetRequest, DeleteRequest};
use tonic::transport::Channel;
use std::time::Duration;  // Correct import for Duration
use tokio::time::sleep;   // Correct import for async sleep

mod kv_store {
    tonic::include_proto!("kv_store");
}

/// A simple client for RustyDB to perform basic operations
pub struct RustyDBClient {
    client: KvStoreClient<Channel>,
}

impl RustyDBClient {
    /// Creates a new client connected to the RustyDB server with retry logic
    pub async fn new(addr: &str) -> Self {
        let mut retries = 5;
        while retries > 0 {
            match KvStoreClient::connect(format!("http://{}:50051", addr)).await {
                Ok(client) => {
                    println!("Successfully connected to the server");
                    return Self { client };
                }
                Err(_) => {
                    retries -= 1;
                    println!("Retrying connection to server... Attempts remaining: {}", retries);
                    sleep(Duration::from_secs(2)).await;  // Now async sleep works
                }
            }
        }
        panic!("Could not connect to the server after several attempts");
    }

    /// Sets a key-value pair in the database
    pub async fn set(&mut self, key: String, value: String) {
        let request = tonic::Request::new(SetRequest { key, value });
        let response = self.client.set(request).await.unwrap();
        println!("Set Response: {:?}", response.into_inner().success);
    }

    /// Retrieves a value from the database using a key
    pub async fn get(&mut self, key: String) {
        let request = tonic::Request::new(GetRequest { key });
        match self.client.get(request).await {
            Ok(response) => println!("Get Response: {:?}", response.into_inner().value),
            Err(status) => eprintln!("Error: {:?}", status),
        }
    }

    /// Deletes a key-value pair from the database
    pub async fn delete(&mut self, key: String) {
        let request = tonic::Request::new(DeleteRequest { key });
        let response = self.client.delete(request).await.unwrap();
        println!("Delete Response: {:?}", response.into_inner().success);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the server using the retry logic
    let mut client = RustyDBClient::new("server").await;

    client.set("foo".to_string(), "bar".to_string()).await;
    client.get("foo".to_string()).await;
    client.delete("foo".to_string()).await;

    Ok(())
}
