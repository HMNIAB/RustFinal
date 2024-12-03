use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const ADDRESS: &str = "127.0.0.1:5000";

fn main() {
    println!("Hello, server!");
}