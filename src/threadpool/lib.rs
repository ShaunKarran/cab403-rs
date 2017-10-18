#[macro_use]
extern crate log;

use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;

// Job is a type alias for a Box that holds a closure (this specific combination of traits is essentially a closure).
// This is because a generic type parameter can only be substituted for 1 concrete type at a time, where as trait
// objects allow for multiple.
type Job = Box<FnBox + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
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
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        debug!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        debug!("Shutting down all workers.");

        for worker in &mut self.workers {
            debug!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let receiver = receiver.lock().unwrap();
                let message = receiver.recv().unwrap(); // blocks

                match message {
                    Message::NewJob(job) => {
                        debug!("Worker {} got a job; executing", id);

                        job.call_box();
                    },
                    Message::Terminate => {
                        debug!("Worker {} was told to terminate", id);

                        break;
                    },
                }
            }
        });

        Worker {
            id: id,
            thread: Some(thread),
        }
    }
}

// I don't really understand it but this is required to be able to move the closure out of the Box<T> and call it.
// From the Rust Book:
// > This is a very sneaky, complicated trick. Don’t worry too much if it doesn’t make perfect sense;
// someday, it will be completely unnecessary.
trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}
