use super::{Request, Response, Route, Router};
use crate::threadpool::ThreadPool;
use std::{
    collections::HashMap,
    io::{BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

pub type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

pub struct Server {
    listener: TcpListener,
    threadpool: ThreadPool,
    router: Arc<Router>,
}

impl Server {
    pub fn new(addr: &str, thread_count: usize, router: Router) -> MyResult<Self> {
        let listener = TcpListener::bind(addr)?;
        let threadpool = ThreadPool::new(thread_count);
        let router = Arc::new(router);

        Ok(Server {
            listener,
            threadpool,
            router,
        })
    }

    pub fn run(self) -> MyResult<()> {
        for stream_result in self.listener.incoming() {
            let router = Arc::clone(&self.router);
            match stream_result {
                Ok(stream) => self.threadpool.execute(move || {
                    if let Err(e) = handle_connection(stream, &router) {
                        eprintln!("Error handling connection: {}", e);
                    }
                }),
                Err(e) => return Err(From::from(format!("Failed to handle connection: {}", e))),
            }
        }
        Ok(())
    }
}

fn handle_connection(mut stream: TcpStream, router: &Router) -> MyResult<()> {
    let mut buf_reader = BufReader::new(stream.try_clone()?);

    loop {
        match Request::from_reader(&mut buf_reader) {
            Ok(request) => {
                let route = Route::new()
                    .http_method(request.method.clone())
                    .path(request.path.clone());
                let response = if let Some(handler) = &router.routes.get(&route) {
                    handler(request.clone())?
                } else {
                    router.not_found_response.clone()
                };
                let should_close = send_response(&mut stream, &response, &request)?;

                if should_close {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading request: {}", e);
                break;
            }
        }
    }

    Ok(())
}

fn send_response(stream: &mut TcpStream, response: &Response, request: &Request) -> MyResult<bool> {
    let http_version = &response.http_version;
    let status_code = response.status_code;
    let reason_phrase = &response.reason_phrase;
    let headers = &response.headers;

    let header = format!(
        "{} {} {}\r\n{}\r\n\r\n",
        http_version,
        status_code,
        reason_phrase,
        headers.as_ref().map(format_headers).unwrap_or_default(),
    );

    stream.write_all(header.as_bytes())?;

    if let Some(body) = &response.body {
        stream.write_all(body)?;
    }

    // Check for the Connection header in the response
    if let Some(connection_header) = request.headers.get("Connection") {
        if connection_header.to_lowercase() == "close" {
            println!("Closing connection");
            stream.shutdown(std::net::Shutdown::Both)?;
            return Ok(true);
        }
    }

    Ok(false)
}

fn format_headers(headers: &HashMap<String, String>) -> String {
    headers
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<String>>()
        .join("\r\n")
}
