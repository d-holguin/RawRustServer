# RustThreadPoolServer

`RustThreadPoolServer` is a multi-threaded web server with session-based authentication and an in-memory database, built in Rust. It does not rely on any third-party libraries and uses only the standard library provided by Rust. This project, inspired by the example in [The Rust Programming Language book](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html), provides a minimal, yet fully-functional, web server. I use this project to deepen my understanding of Rust, its concurrency features, and the underlying principles of HTTP. 
## Table of Contents

- [RustThreadPoolServer](#rustthreadpoolserver)
- [Usage](#usage)
- [ThreadPool](#threadpool)
- [Error Handling](#error-handling)
- [SimpleDB](#database)
## Usage
In this extended example, we construct a web server that implements session-based authentication, routing, and an in-memory database. To achieve this, we create a Router object and add several routes each with corresponding handler functions. Upon a successful login attempt in PostLoginHandler, a new session is created and stored in the database, and the client is issued a cookie containing the session ID. On subsequent requests, the HomeHandler checks the session status. If the session is authenticated, it serves the homepage; otherwise, it redirects the client to the login page.
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
    database: Arc<Database>,
}
impl RouteHandler for PostLoginHandler {
    fn handle(&self, request: Request) -> Result<Response, AnyErr> {
        let err_login = include_str!("../assets/error-login.html").to_string();
        if let Some(form_data) = request.form_urlencoded() {
            let error_response = Ok(ResponseBuilder::new()
                .content_type(ContentType::Html)
                .body_string(err_login)
                .build());
            let username = get_form_value(&form_data, "username");
            let password = get_form_value(&form_data, "password");

            if username.is_none() || password.is_none() {
                return error_response;
            }

            match self.database.users.get(username.unwrap().to_string())? {
                Some(user) if &user.password == password.unwrap() => {
                    // Login successful
                    println!("User Login: {:?}", user);
                    let session_id = Session::generate_session_id();
                    let session = Session {
                        username: user.username.clone(),
                        session_id: session_id.clone(),
                        last_active: Instant::now(),
                    };
                    self.database.sessions.insert(session_id.clone(), session)?;
                    return Ok(ResponseBuilder::new()
                        .cookie(Cookie::new("session_id".to_string(), session_id))
                        .temp_redirect("/home")
                        .build());
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

impl RouteHandler for HomeHandler {
    fn handle(&self, request: Request) -> Result<Response, AnyErr> {
        let body = include_str!("../assets/home.html").to_string();

        let login_redirect = ResponseBuilder::new().temp_redirect("/login").build();

        match self.authenticate_session(request)? {
            AuthResult::Authenticated => Ok(ResponseBuilder::new()
                .content_type(ContentType::Html)
                .body_string(body)
                .build()),
            AuthResult::SessionNotPresent => Ok(login_redirect),
            AuthResult::SessionInvalid => Ok(login_redirect),
        }
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