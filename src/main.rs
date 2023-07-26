use web_server::server;

fn main() {
    if let Err(err) = server::run() {
        eprintln!("{}", err);
    }
}
