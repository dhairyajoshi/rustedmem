use std::{
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    sender: mpsc::Sender<Job>,
    workers: Vec<Worker>,
}
impl ThreadPool {
    pub fn new(cap: i32) -> ThreadPool {
        assert!(cap > 0);

        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::new();
        let reciever = Arc::new(Mutex::new(receiver));
        for i in 0..cap {
            workers.push(Worker::new(i, Arc::clone(&reciever)));
        }
        ThreadPool { sender, workers }
    }

    pub fn execute<T>(&self, job: T)
    where
        T: FnOnce() + Send + 'static,
    {
        let job = Box::new(job);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: i32,
    thread: JoinHandle<()>,
}
impl Worker {
    fn new(id: i32, reciever: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let guard = reciever.lock().unwrap().recv();
            match guard {
                Ok(job) => job(),
                Err(_) => break,
            }
        });
        Worker { id, thread }
    }
}
