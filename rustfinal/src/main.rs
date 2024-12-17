use env_logger;
use log::{error, info};
use reqwest::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio;

/// A struct representing the server with a shared counter to track the number of handled requests.
#[derive(Clone)]
struct Server {
    counter: Arc<Mutex<i32>>, // Shared state using Mutex and Arc to enable safe concurrent access.
}

impl Server {
    /// Creates a new `Server` instance with an initial counter value of 0.
    ///
    /// # Returns
    /// A new `Server` instance.
    fn new() -> Self {
        Server {
            counter: Arc::new(Mutex::new(0)),
        }
    }

    /// Simulates handling a request. It sleeps for 100ms to mimic processing time
    /// and increments a shared counter to track the number of handled requests.
    ///
    /// If the mutex is poisoned (i.e., the lock was poisoned due to a panic), it will recover and continue.
    async fn handle_request(&self) {
        // Simulate request handling with a sleep of 100ms
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        match self.counter.lock() {
            Ok(mut counter) => {
                // Increment the counter if the lock is acquired successfully
                *counter += 1;
                info!("Handled request, counter: {}", *counter);
            }
            Err(poisoned) => {
                // Handle the case where the mutex is poisoned (i.e., a panic occurred while holding the lock)
                error!("Mutex was poisoned, recovering...");
                let mut counter = poisoned.into_inner();
                *counter += 1;
                info!("Handled request after poisoning, counter: {}", *counter);
            }
        }
    }

    /// Starts handling a specified number of concurrent requests. 
    ///
    /// # Arguments
    /// * `num_requests`: The number of requests to handle concurrently.
    ///
    /// This function spawns tasks to handle each request concurrently using `tokio::spawn`.
    async fn start(&self, num_requests: usize) {
        let mut tasks = vec![];
        for _ in 0..num_requests {
            let server_clone = self.clone();
            let task = tokio::spawn(async move {
                server_clone.handle_request().await;
            });
            tasks.push(task);
        }
        for task in tasks {
            // Wait for all tasks to complete
            task.await.unwrap();
        }
    }

    /// Makes a GET request to the specified URL using the `reqwest` client.
    ///
    /// # Arguments
    /// * `url`: The URL to send the GET request to.
    ///
    /// # Returns
    /// A result indicating success or failure. If the request fails, the error is logged.
    async fn make_request(&self, url: &str) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;

        if response.status().is_success() {
            info!(
                "Request to {} completed with status: {}",
                url,
                response.status()
            );
        } else {
            error!(
                "Request to {} failed with status: {}",
                url,
                response.status()
            );
        }
        Ok(())
    }
}

/// The main entry point of the program, where the server is initialized and requests are handled.
/// The logger is initialized, and the server starts processing 500 requests concurrently.
#[tokio::main]
async fn main() {
    env_logger::init(); // Initialize the logger
    info!("Program started.");
    let server = Server::new();
    info!("Server initialized.");
    server.start(500).await; // Handle 500 requests concurrently
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    /// Test that the server correctly handles requests and increments the counter.
    #[tokio::test]
    async fn test_handle_request() {
        let server = Server::new();
        server.start(5).await;
        let counter = server.counter.lock().unwrap();
        assert_eq!(
            *counter, 5,
            "Counter should be 5 after handling 5 requests."
        );
    }

    /// Test that a successful request to a valid URL completes successfully.
    #[tokio::test]
    async fn test_make_request() {
        let server = Server::new();
        let result = server.make_request("https://www.rust-lang.org").await;
        assert!(result.is_ok(), "Request should succeed");
    }

    /// Test that a request to an invalid URL fails as expected.
    #[tokio::test]
    async fn test_make_request_failure() {
        let server = Server::new();
        let result = server.make_request("https://notaurl.url").await;
        assert!(!result.is_ok(), "Request should fail but pass test");
    }

    /// Test that multiple concurrent requests correctly increment the counter.
    #[tokio::test]
    async fn test_multiple_concurrent_requests() {
        let server = Server::new();
        let num_requests = 100;
        server.start(num_requests).await;
        let counter = server.counter.lock().unwrap();
        assert_eq!(
            *counter, num_requests as i32,
            "Counter should be {}",
            num_requests
        );
    }
}

