use log::{info};
use env_logger;
use std::sync::{Arc, Mutex};
use tokio;

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
}

#[tokio::main]
async fn main() {
    env_logger::init(); //initialize the logger
    info!("Program started.");
    let server = Server::new();
    info!("Sever initialized.");
}
