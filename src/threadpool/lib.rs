use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // Create a channel to use to send Jobs to the Workers.
        let (sender, receiver) = mpsc::channel();

        // Arc will let multiple workers own the receiver.
        // Mutex will make sure that only one worker is getting a job from the receiver at a time.
        // Mutex is needed because a worker must mutate the reciever when recieving the Job.
        let receiver = Arc::new(Mutex::new(receiver));

        // Create a worker for each thread in the pool.
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()))
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&mut self, function: F)
        where F: FnOnce() + Send + 'static
    {
        // Create the job. A Job is just a Box that contains a closure.
        let job = Box::new(function);

        // Send the job down the channel for the next available worker to execute.
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let receiver = receiver.lock().unwrap();
                let job = receiver.recv().unwrap(); // blocks

                println!("Worker {} got a job; executing.", id);

                (*job)();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}

// Job is a type alias for a Box that holds a closure (this specific combination of traits is essentially a closure).
// This is done to simplfy the rest of the code as we dont need to specify a generic type to represent a closure on
// the ThreadPool and the Worker.
type Job = Box<FnOnce() + Send + 'static>;
