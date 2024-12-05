```rust
println!("Rust Final Project - Avery Beauter");
```
### Project Overview
This is an implementation of a multi-threaded web server that loads a simple HTML 'homepage' when the server receives a request from a client. The server is designed to handle multiple clients and requests. 

### Motivation 
I elected to create a multi-threaded server as my interests as of late has been Web-Development with focus on Ruby on Rails and Go. While all of these languages have a purpose - I wanted to examine how Rust compares on the backend to a language such as Go. As both Rust and Go have their own methods of implementing concurrency it is interesting to see where each thrives and any potential shortcomings. 

### Design and Architecture
image here -> 

#### Application Structure
The server utilizes the following crates: 

#### Technology Stack
Here will be an explanation of the tools, libraries, and frameworks used in conjunction with Rust

#### Data Flow

### Lessons Learned
Rust is not an easy language to use as a beginner. While each language touts their own set of robust features, ease of use, etc Rust offers memory safety and compile time guarantees. While this may be worth its weight in gold in a setting with critical systems, the guarantees that are given from the language are overshadowed by the development time and general headaches to a novice programmer. While Go is not an end all be all language, it provides near identical speeds with far simpler mechanisms to implement a web-server. Additionally, Go is much faster to develop in and has a significant amount of literature out there for the novice programmer.

### Future Enhancements
The project can be enhanced to handle POST/GET requests with more than a single route. Furthermore, the website itself could be fleshed out - say - as an image board that would utilize a database and offer a state that could be built upon.
