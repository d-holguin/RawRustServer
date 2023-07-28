use std::{
    collections::HashMap,
    io::{BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
};
use crate::{server::ResponseBuilder, threadpool::ThreadPool};
use super::{Request, Response};


pub type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

pub struct Server {
    listener: TcpListener,
    threadpool: ThreadPool,
    routes: Arc<HashMap<String, Box<dyn Fn(Request) -> MyResult<Response> + Send + Sync>>>,
}

impl Server {
    pub fn new(addr: &str, thread_count: usize) -> MyResult<Self> {
        let listener = TcpListener::bind(addr)?;
        let threadpool = ThreadPool::new(thread_count);
        let routes = Arc::new(HashMap::new());

        Ok(Server {
            listener,
            threadpool,
            routes,
        })
    }

    pub fn add_route<F>(&mut self, endpoint: String, handler: F)
    where
        F: Fn(Request) -> MyResult<Response> + 'static + Send + Sync,
    {
        Arc::get_mut(&mut self.routes)
            .unwrap()
            .insert(endpoint, Box::new(handler));
    }
    pub fn run(self) -> MyResult<()> {
        for stream_result in self.listener.incoming() {
            let routes = Arc::clone(&self.routes); 
            match stream_result {
                Ok(stream) => self.threadpool.execute(move || {
                    if let Err(e) = handle_connection(stream, routes) {
                        eprintln!("Error handling connection: {}", e);
                    }
                }),
                Err(e) => return Err(From::from(format!("Failed to handle connection: {}", e))),
            }
        }
        Ok(())
    }
}

fn handle_connection(
    mut stream: TcpStream,
    routes: Arc<HashMap<String, Box<dyn Fn(Request) -> MyResult<Response> + Send + Sync>>>,
) -> MyResult<()> {
    let mut buf_reader = BufReader::new(stream.try_clone()?);

    loop {
        match Request::from_reader(&mut buf_reader) {
            Ok(request) => {
                println!("Received request: {:?}", request);

                let response = if let Some(handler) = routes.get(&request.path) {
                    handler(request.clone())?
                } else {
                    let mut headers = HashMap::new();
                    let page_404 = std::fs::read_to_string("assets/404.html")?;
                    headers.insert("Content-Type".to_string(), "text/html".to_string());
                    headers.insert("Content-Length".to_string(), page_404.len().to_string());

                    ResponseBuilder::new()
                        .status_code(404)
                        .headers(headers)
                        .body(page_404.into_bytes())
                        .build()
                };
                send_response(&mut stream, &response)?;

                match request.headers.get("Connection") {
                    Some(value) if value.to_lowercase() == "close" => {
                        // Close the connection if the header is 'close'.
                        stream.shutdown(std::net::Shutdown::Both)?;
                        break;
                    }
                    _ => {
                        // Do nothing, keep the connection alive by default.
                    }
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

fn send_response(stream: &mut TcpStream, response: &Response) -> MyResult<()> {
    let http_version = &response.http_version;
    let status_code = response.status_code;
    let reason_phrase = &response.reason_phrase;
    let headers = &response.headers;
    let body = &response.body;

    let response = format!(
        "{} {} {}\r\n{}\r\n\r\n{}",
        http_version,
        status_code,
        reason_phrase,
        headers
            .as_ref()
            .map(|h| format_headers(h))
            .unwrap_or_default(),
        String::from_utf8_lossy(body.as_ref().unwrap_or(&vec![]))
    );

    stream.write_all(response.as_bytes())?;

    Ok(())
}

fn format_headers(headers: &HashMap<String, String>) -> String {
    headers
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<String>>()
        .join("\r\n")
}
