# RustThreadPoolServer

`RustThreadPoolServer` is a multi-threaded web server built with Rust. It does not rely on any third-party libraries and uses only the standard library provided by Rust. This project, inspired by the example in [The Rust Programming Language book](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html), provides a minimal, yet fully-functional, web server. I use this project to deepen my understanding of Rust and its concurrency features, and the underlying principles of HTTP. 
## Usage

Here's how you can implement a simple web server using this project:

```rust
fn main() {
    match run_server() {
        Ok(_) => println!("Server shut down successfully."),
        Err(e) => {
            eprintln!("Failed to run server: {:?}", e);
            std::process::exit(1);
        }
    }
}
fn run_server() -> Result<(), AnyErr> {
    let router = Router::new()
        .add_route(HttpMethod::get("/"), home)
        .add_route(HttpMethod::get("/favicon.ico"), favicon);

    let server = ServerBuilder::new()
        .address("127.0.0.1:8000")
        .thread_count(4)
        .router(router)
        .build()?;
    server.run()
}

fn home(_request: Request) -> Result<Response, AnyErr> {
    let body = std::fs::read_to_string("assets/home.html")?;

    Ok(ResponseBuilder::new()
        .content_type(ContentType::Html)
        .body_string(body)
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
```

## Error Handling
The project leverages a custom error type, `AnyErr`, for flexible and efficient error handling. `AnyErr` can wrap any error, providing additional context while maintaining the original error as the source. This enables better error tracking and easier debugging. Inspired by [anyhow](https://github.com/dtolnay/anyhow).

Here's a look at `AnyErr`:
```rust
pub struct AnyErr {
    message: String,
    source: Option<Box<dyn Error>>,
}

impl AnyErr {
    pub fn new<M: Into<String>>(message: M) -> Self {
        //...
    }

    pub fn wrap<E: Error + 'static>(message: String, error: E) -> Self {
        //...
    }
}
```