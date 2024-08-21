use std::sync::{Arc, mpsc, Mutex};
use std::thread;

pub struct ThreadPool{
   workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

struct Job;

impl Worker{
    fn new (id:usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>)->Worker{
        let thread = thread::spawn(||{
            receiver;
        });

        Worker{id,thread}
    }
}

impl ThreadPool{

    // Create a new ThreadPool.
    //
    pub fn new(size:usize) -> ThreadPool{
        assert!(size>0);

        let (sender ,receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size{
            //     create workers
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool{  workers, sender }
    }
    pub fn execute<F>(&self, f:F)
    where
        F:FnOnce()+Send+'static{

        let job  = Box::new(f);
        self.sender.send(job).unwrap();

    }
}