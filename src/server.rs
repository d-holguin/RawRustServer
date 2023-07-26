use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use crate::threadpool::ThreadPool;

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn run() -> MyResult<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let threadpool = ThreadPool::new(2);

    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => threadpool.execute(|| {
                handle_connection_wrapped(stream);
            }),
            Err(e) => return Err(From::from(format!("Failed to handle connection: {}", e))),
        }
    }

    Ok(())
}

struct RouteResponse {
    status_line: &'static str,
    filename: &'static str,
}
impl RouteResponse {
    fn new(status_line: &'static str, filename: &'static str) -> Self {
        RouteResponse {
            status_line,
            filename,
        }
    }
}

enum Route {
    Root,
    Sleep,
    NotFound,
}

impl Route {
    fn from_request_line(request_line: &str) -> Self {
        match request_line {
            "GET / HTTP/1.1" => Route::Root,
            "GET /sleep HTTP/1.1" => Route::Sleep,
            _ => Route::NotFound,
        }
    }

    fn get_response(&self) -> RouteResponse {
        match self {
            Route::Root => RouteResponse::new("HTTP/1.1 200 OK", "assets/hello.html"),

            Route::Sleep => {
                std::thread::sleep(std::time::Duration::from_secs(5));
                RouteResponse::new("HTTP/1.1 200 OK", "assets/hello.html")
            }
            Route::NotFound => RouteResponse::new("HTTP/1.1 404 NOT FOUND", "assets/404.html"),
        }
    }

    fn respond(self, mut stream: TcpStream) -> MyResult<()> {
        let response_config = self.get_response();

        let contents = fs::read_to_string(response_config.filename)?;
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            response_config.status_line,
            contents.len(),
            contents
        );
        stream.write_all(response.as_bytes())?;
        Ok(())
    }
}

fn handle_connection(mut stream: TcpStream) -> MyResult<()> {
    let buf_reader = BufReader::new(&mut stream);

    match buf_reader.lines().next() {
        Some(Ok(request_line)) => {
            println!("Received request line: {}", request_line);
            Route::from_request_line(&request_line).respond(stream)?;
        }
        Some(Err(e)) => {
            eprintln!("Error reading request line: {}", e);
        }
        None => {
            eprintln!("No request line received");
        }
    }

    Ok(())
}

fn handle_connection_wrapped(stream: TcpStream) {
    match handle_connection(stream) {
        Ok(()) => (),
        Err(e) => eprintln!("Error handling connection: {}", e),
    }
}
