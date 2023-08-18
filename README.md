# RustThreadPoolServer
`RustThreadPoolServer` is a multi-threaded web server built in Rust, featuring session-based authentication, an in-memory database, wildcard image support, and a `Galactic Bounty Hunter` theme. Using only the Rust standard library and no third-party dependencies, this project serves as a hands-on exploration of Rust's concurrency features and HTTP principles.
## Table of Contents
- [RustThreadPoolServer](#rustthreadpoolserver)
- [Usage](#usage)
- [ThreadPool](#threadpool)
- [Error Handling](#error-handling)
- [SimpleDB](#database)
- [Logger Utility](#logger-utility)

## Usage

![Usage Example](https://github.com/d-holguin/RustThreadPoolServer/blob/main/example/usage-example.gif)
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
    let database = Arc::new(Database::database_init()?);
    let router = Router::new()
        .add_route(
            HttpMethod::get("/home"),
            HomeHandler {
                database: Arc::clone(&database),
            },
        )
        .add_route(HttpMethod::get("/styles.css"), CssHandler)
        .add_route(HttpMethod::get("/favicon.ico"), FaviconHandler)
        .add_route(
            HttpMethod::post("/login"),
            PostLoginHandler {
                database: Arc::clone(&database),
            },
        )
        .add_route(HttpMethod::get("/images/*"), GetImageHandler)
        .add_route(HttpMethod::get("/login"), GetLoginHandler);

    let server = ServerBuilder::new()
        .address("127.0.0.1:8000")
        .thread_count(4)
        .router(router)
        .build()?;
    server.run()
}

// get.rs in the login module
pub struct GetLoginHandler;
impl RouteHandler for GetLoginHandler {
    fn handle(&self, _request: Request) -> Result<Response, AnyErr> {
        let html = include_str!("./login.html").to_string();

        Ok(ResponseBuilder::new()
            .content_type(ContentType::Html)
            .body_string(html)
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

`RustThreadPoolServer` includes a simple in-memory database for handling user and session data.

```rust
pub struct Database {
    pub users: Arc<SimpleDB<String, User>>,
    pub sessions: Arc<SimpleDB<String, Session>>,
}
impl Database {
    pub fn database_init() -> Result<Database, AnyErr> {
        let users: Arc<SimpleDB<String, User>> = Arc::new(SimpleDB::new());
        let sessions: Arc<SimpleDB<String, Session>> = Arc::new(SimpleDB::new());
        let admin_user = User {
            username: "admin".to_string(),
            password: "hunter12".to_string(),
        };
        users
            .insert(admin_user.username.clone(), admin_user)
            .map_err(|e| AnyErr::wrap("error adding admin credentials to database", e))?;

        Ok(Database { users, sessions })
    }
}
 ```

## Error Handling
`AnyErr` serves as a straightforward mechanism for wrapping contextual information around errors, making them more descriptive and easier to debug.
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
## Logger Utility

The `Logger Utility` provides a simple and efficient way to log messages both to the terminal and to a file. The logger supports different log levels such as <span style="color:green">INFO</span> and <span style="color:red">ERROR</span>.

When logging an `INFO` message, it will appear in the terminal as:
```plaintext
<span style="color:green">[INFO]</span> Starting Server...
```
To use the logger, first initialize the global logger backend with the desired log file name:


```rust
logger_backend::init_global_logger("log.txt");
```
Then you can use the logger anywhere in code like this:

```
logger::info("Starting Server...");
```