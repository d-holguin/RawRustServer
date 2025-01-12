# RawRustServer
`RawRustServer`  is a simple multithreaded web server built in Rust for fun and self-learning. It originally expanded on the Rust book's Multi-Threaded Web Server project, which you can find here: https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html 
This only uses the rust standard library which doesn't include a http server, logger, threadpool, etc.


## Usage

![Usage Example](/example/usage-example.gif)
```rust
fn run_server() -> Result<()> {
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
``````

## Logger Utility

The `Logger Utility` provides a simple and efficient way to log messages both to the terminal and to a file. The logger supports different log levels such as <span style="color:green">INFO</span> and <span style="color:red">ERROR</span>.

When logging an `INFO` message, it will appear in the terminal as:

<span style="color:green">[INFO]</span> Starting Server...

To use the logger, first initialize the global logger backend with the desired log file name:


```rust
logger_backend::init_global_logger("logs/server.log");
```
Then you can use the logger anywhere in code like this:

```
logger::info("Starting Server...");
```