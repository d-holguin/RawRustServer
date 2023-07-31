use super::{Request, Response, Route, Router};
use crate::{threadpool::ThreadPool, utils::AnyErr};
use std::{
    collections::HashMap,
    io::{BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
    time::Duration,
};

pub struct Server {
    pub listener: TcpListener,
    pub threadpool: ThreadPool,
    pub router: Arc<Router>,
}

impl Server {
    pub fn run(self) -> Result<(), AnyErr> {
        println!("Server started");
        for stream_result in self.listener.incoming() {
            let router = Arc::clone(&self.router);
            match stream_result {
                Ok(stream) => self.threadpool.execute(move || {
                    if let Err(e) = handle_connection(stream, &router) {
                        eprintln!("Error handling connection: {}", e);
                    }
                }),
                Err(e) => return Err(AnyErr::new(format!("Failed to handle connection {}", e))),
            }
        }
        Ok(())
    }
}
enum ResponseStatus {
    Done,
    Continue,
}
fn handle_connection(mut stream: TcpStream, router: &Router) -> Result<(), AnyErr> {
    let mut buf_reader = BufReader::new(stream.try_clone()?);

    loop {
        println!("Handling a request");
        stream
            .set_read_timeout(Some(Duration::from_secs(60)))
            .expect("Failed to set read timeout");
        match Request::from_reader(&mut buf_reader) {
            Ok(request) => {
                let route = Route::new()
                    .http_method(request.method.clone())
                    .path(request.path.clone());

                let response = match &router.routes.get(&route) {
                    Some(handler) => handler(request.clone())?,
                    None => router.not_found_response.clone(),
                };
                match send_response(&mut stream, &response, &request)? {
                    ResponseStatus::Done => break,
                    ResponseStatus::Continue => continue,
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

fn send_response(
    stream: &mut TcpStream,
    response: &Response,
    request: &Request,
) -> Result<ResponseStatus, AnyErr> {
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
    //let connection_header = request.headers.get("Connection").map(|s| s.to_lowercase());

    Ok(ResponseStatus::Done)
}

fn format_headers(headers: &HashMap<String, String>) -> String {
    headers
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<String>>()
        .join("\r\n")
}
