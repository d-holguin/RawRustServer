# RustThreadPoolServer

`RustThreadPoolServer` is a multi-threaded web server built with Rust. This project, inspired by the example in The Rust Programming Language book, provides a minimal, yet fully-functional, web server. I use this project to deepen my understanding of Rust and its concurrency features, and the underlying principles of HTTP.

## Usage

Here's how you can implement a simple web server using this project:

```rust
use std::{collections::HashMap, fs::File, io::Read};
use web_server::server::{MyResult, Request, Response, ResponseBuilder, Server};

fn main() {
    match Server::new("127.0.0.1:8000", 4) {
        Ok(mut server) => {
            server.add_route("/home".to_string(), home);
            if let Err(e) = server.run() {
                eprint!("Server error: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to start server: {}", e),
    }
}

fn home(_request: Request) -> MyResult<Response> {
    let mut headers = HashMap::new();

    let body = std::fs::read_to_string("assets/home.html")?;
    headers.insert("Content-Type".to_string(), "text/html".to_string());
    headers.insert("Content-Length".to_string(), body.len().to_string());
    
    Ok(ResponseBuilder::new()
        .headers(headers)
        .body(body.into_bytes())
        .build())
}


``````
## ThreadPool

The `ThreadPool` module is the heart of the server's ability to handle multiple connections concurrently. It maintains a pool of worker threads and provides an interface to execute jobs on these threads. This design allows the server to manage workload effectively and prevent individual requests from blocking the entire server.

Here's a brief look at `ThreadPool`:

```rust
impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }
    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
