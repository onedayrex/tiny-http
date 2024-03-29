use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;

pub struct  ThreadPool{
    workers:Vec<Worker>,
    sender:mpsc::Sender<Job>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(thread_num: usize) -> ThreadPool {
        assert!(thread_num>0);

        let (sender,receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(thread_num);
        for id in 0..thread_num {
            //create thread and put it to vec
            let worker = Worker::new(id,Arc::clone(&receiver));
            workers.push(worker);
        }

        ThreadPool{workers,sender}

    }

    pub fn execute<F>(&self,f:F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap()
    }
}

struct Worker{
    id:usize,
    thread:thread::JoinHandle<()>
}

impl Worker {
    fn new(id:usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker{
        let thread = thread::spawn(move|| loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job,executing...");
            job();
        });
        Worker{
            id,
            thread
        }
    }
}