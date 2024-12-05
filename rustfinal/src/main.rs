use log::{info};
use env_logger;
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
}

#[tokio::main]
async fn main() {
    env_logger::init(); //initialize the logger
    info!("Program started.");
    let server = Server::new();
    info!("Sever initialized.");
    server.handle_request().await; //handle simmed request
}
