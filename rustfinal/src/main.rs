use log::{info, error};
use env_logger;
use reqwest::Error;
use std::sync::{Arc, Mutex};
use tokio;
use std::time::Duration;

#[derive(Clone)]
struct Server {
    counter: Arc<Mutex<i32>>, // Shared state using Mutex and Arc
}

impl Server {
    fn new() -> Self {
        Server {
            counter: Arc::new(Mutex::new(0)),
        }
    }
    async fn handle_request(&self) { //sim request handling with a sleep of 100ms
        tokio::time::sleep(Duration::from_millis(100)).await;
        let mut counter = self.counter.lock().unwrap(); //lock the counter and increment
        *counter += 1;
        info!("Handled request, counter: {}", *counter);
    }
    async fn start(&self, num_requests: usize) { //usize to prevent overflow
        let mut tasks = vec![];
        for _ in 0..num_requests {
            let server_clone = self.clone();
            let task = tokio::spawn(async move {
                server_clone.handle_request().await;
            });
            tasks.push(task);
        }
        for task in tasks { //wait for all tasks
            task.await.unwrap();
        }
    }
    async fn make_request(&self, url: &str) -> Result<(),Error> {
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;
        
        if response.status().is_success() {
            info!("Request to {} completed with status: {}", url, response.status());
        } else {
            error!("Request to {} failed with status: {}", url, response.status());
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    env_logger::init(); //initialize the logger
    info!("Program started.");
    let server = Server::new();
    info!("Sever initialized.");
    server.start(5).await; //handling 5 requests concurrently    //server.handle_request().await; //handle simmed request

}
