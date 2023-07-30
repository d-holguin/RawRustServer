use std::{net::TcpListener, sync::Arc};

use crate::{threadpool::ThreadPool, utils::AnyErr};

use super::{Router, Server};

pub struct ServerBuilder {
    address: Option<String>,
    thread_count: Option<usize>,
    router: Option<Router>,
}
impl Default for ServerBuilder {
    fn default() -> Self {
        Self {
            address: Some("127.0.0.1:8000".to_string()),
            thread_count: Some(2),
            router: None,
        }
    }
}

impl ServerBuilder {
    pub fn new() -> Self {
        ServerBuilder::default()
    }
    pub fn address(mut self, address: impl ToString) -> Self {
        self.address = Some(address.to_string());
        self
    }
    pub fn thread_count(mut self, thread_count: usize) -> Self {
        self.thread_count = Some(thread_count);
        self
    }
    pub fn router(mut self, router: Router) -> Self {
        self.router = Some(router);
        self
    }
    pub fn build(self) -> Result<Server, AnyErr> {
        let address = self.address.ok_or(AnyErr::new("Address is missing"))?;
        let thread_count = self
            .thread_count
            .ok_or(AnyErr::new("Thread count is missing"))?;
        let router = self.router.ok_or(AnyErr::new("Router is missing"))?;

        let listener = TcpListener::bind(address)?;
        let threadpool = ThreadPool::new(thread_count);
        let router = Arc::new(router);
        Ok(Server {
            listener,
            threadpool,
            router,
        })
    }
}
