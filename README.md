# RustThreadPoolServer

`RustThreadPoolServer` is a multi-threaded web server and in-memory database built with Rust. It does not rely on any third-party libraries and uses only the standard library provided by Rust. This project, inspired by the example in [The Rust Programming Language book](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html), provides a minimal, yet fully-functional, web server. I use this project to deepen my understanding of Rust, its concurrency features, and the underlying principles of HTTP. 
## Table of Contents

- [RustThreadPoolServer](#rustthreadpoolserver)
- [Usage](#usage)
- [ThreadPool](#threadpool)
- [Error Handling](#error-handling)
- [SimpleDB](#database)
## Usage
In this example, we implement a simple login page with GET and POST handler functions. Once a successful login is processed, a redirect response is sent:
![Usage Example](https://github.com/d-holguin/RustThreadPoolServer/blob/main/example/usage-example.gif)
```rust
use std::sync::Arc;
use web_server::database::SimpleDB;
use web_server::http_server::RouteHandler;
use web_server::{
    http_server::{
        ContentType, HttpMethod, Request, Response, ResponseBuilder, Router, ServerBuilder,
    },
    utils::AnyErr,
};

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
    let database: Arc<SimpleDB<String, String>> = Arc::new(SimpleDB::new());
    database.insert("admin".to_string(), "hunter12".to_string())?;
    let router = Router::new()
        .add_route(HttpMethod::get("/home"), HomeHandler)
        .add_route(HttpMethod::get("/favicon.ico"), FaviconHandler)
        .add_route(
            HttpMethod::post("/login"),
            PostLoginHandler {
                database: Arc::clone(&database),
            },
        )
        .add_route(HttpMethod::get("/login"), GetLoginHandler);

    let server = ServerBuilder::new()
        .address("127.0.0.1:8000")
        .thread_count(4)
        .router(router)
        .build()?;
    server.run()
}
struct PostLoginHandler {
    database: Arc<SimpleDB<String, String>>,
}
impl RouteHandler for PostLoginHandler {
    fn handle(&self, request: Request) -> Result<Response, AnyErr> {
        let err_login = include_str!("../assets/error-login.html").to_string();
        if let Some(form_data) = request.form_data {
            let error_response = Ok(ResponseBuilder::new()
                .content_type(ContentType::Html)
                .body_string(err_login)
                .build());
            let username = match form_data.get("username") {
                Some(username) => username,
                None => {
                    return error_response;
                }
            };

            let password = match form_data.get("password") {
                Some(password) => password,
                None => {
                    return error_response;
                }
            };

            match self.database.get(username.to_string())? {
                Some(stored_password) if &stored_password == password => {
                    // Login successful
                    return Ok(ResponseBuilder::new().temp_redirect("/home").build());
                }
                _ => {
                    return error_response;
                }
            }
        }

        Ok(ResponseBuilder::new()
            .content_type(ContentType::Html)
            .body_string(err_login)
            .build())
    }
}
struct GetLoginHandler;
impl RouteHandler for GetLoginHandler {
    fn handle(&self, _request: Request) -> Result<Response, AnyErr> {
        let html = include_str!("../assets/login.html");

        Ok(ResponseBuilder::new()
            .content_type(ContentType::Html)
            .body_string(html.to_string())
            .build())
    }
}

struct HomeHandler;
impl RouteHandler for HomeHandler {
    fn handle(&self, _request: Request) -> Result<Response, AnyErr> {
        let body = include_str!("../assets/home.html").to_string();

        Ok(ResponseBuilder::new()
            .content_type(ContentType::Html)
            .body_string(body)
            .build())
    }
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
## Database

`RustThreadPoolServer` includes a simple in-memory database for handling user data. The `SimpleDB` struct is used to store usernames and passwords, and is queried whenever a user tries to log in.

The database is implemented as a shared, thread-safe data structure using `Arc<SimpleDB<String, String>>`. This allows multiple instances of `PostLoginHandler` to have access to the same database data across multiple threads.

In the given example, the `SimpleDB` is initialized with a single user having username "admin" and password "hunter12". 

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

//Example usage 
if let Some(len) = headers.get("content-length") {
    let len: usize = len
        .parse::<usize>()
        .map_err(|e| AnyErr::wrap("Error parsing content length".to_string(), e))?;
    body.reserve(len);
    while body.len() < len {
        let buffer = reader
            .fill_buf()
            .map_err(|e| AnyErr::wrap("Error reading request body".to_string(), e))?;
        let bytes_to_read = std::cmp::min(buffer.len(), len - body.len());
        body.extend_from_slice(&buffer[..bytes_to_read]);
        reader.consume(bytes_to_read);
    }
}

```