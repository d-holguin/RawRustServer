# RustThreadPoolServer

`RustThreadPoolServer` is a multi-threaded web server built with Rust. This project, inspired by the example in The Rust Programming Language book, provides a minimal, yet fully-functional, web server. I use this project to deepen my understanding of Rust and its concurrency features, and the underlying principles of HTTP.

## Usage

Here's how you can implement a simple web server using this project:

```rust
fn main() {
    let router = Router::new()
        .add_route(HttpMethod::get("/home"), home)
        .add_route(HttpMethod::get("/favicon.ico"), favicon);
    match Server::new("127.0.0.1:8000", 4, router) {
        Ok(server) => {
            if let Err(e) = server.run() {
                eprint!("Server error: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to start server: {}", e),
    }
}

fn home(_request: Request) -> MyResult<Response> {
    let body = std::fs::read_to_string("assets/home.html")?;

    Ok(ResponseBuilder::new()
        .content_type(ContentType::Html)
        .body_string(body)
        .build())
}

fn favicon(_request: Request) -> MyResult<Response> {
    let body = std::fs::read("assets/favicon.ico")?;
    Ok(ResponseBuilder::new()
        .content_type(ContentType::Ico)
        .body_bytes(body)
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
        self.sender.send(Message::Job(job)).unwrap();
    }
}
