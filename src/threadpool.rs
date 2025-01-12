use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

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

        if let Err(e) = self.sender.send(Message::Job(job)) {
            eprintln!("{}", format!("Error sending message in threadpool {:?}", e));
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    Job(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            if let Err(e) = self.sender.send(Message::Terminate) {
                eprintln!("{}", format!("Error sending terminate message {:?}", e));
            }
        }
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                if let Err(e) = thread.join() {
                    eprintln!("{}", format!("Error joining worker thread {} {:?}",worker.id, e));
                }
            }
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(Message::Job(job)) => {
                    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| job()));
                    if let Err(e) = result {
                        let error_message = format!("Job panicked: {:?}", e);
                        eprintln!("{}", &error_message);
                    }
                }
                Ok(Message::Terminate) => {
                    println!("Terminating worker {}", id);
                    break;
                }
                Err(e) => {
                    println!("Error {e} worker {id} disconnected; Shutting down.");
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
