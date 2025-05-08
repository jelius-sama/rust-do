// src/libs/thread_pool.rs

//! A simple, robust thread‑pool for executing `FnOnce() + Send + 'static` jobs.
//!
//! # Example
//!
//! ```rust
//! use my_project::ThreadPool;
//!
//! fn main() {
//!     let pool = ThreadPool::new(4);
//!     for i in 0..8 {
//!         pool.execute(move || {
//!             println!("Hello from task {}", i);
//!         });
//!     }
//!     // When `pool` goes out of scope, it will gracefully shut down.
//! }
//! ```

use std::{
    sync::{Arc, Mutex, mpsc},
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

/// Messages sent over the channel to workers.
pub enum Message {
    NewJob(Job),
    Terminate,
}

/// A pool of worker threads for executing jobs concurrently.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new `ThreadPool` with the given number of worker threads.
    ///
    /// # Panics
    ///
    /// Panics if `size == 0`.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "ThreadPool size must be > 0");

        let (sender, receiver) = mpsc::channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /// Submit a job to be executed by the pool.
    ///
    /// If the pool has already been shut down (dropped), this will return an error.
    pub fn execute<F>(&self, job: F) -> Result<(), mpsc::SendError<Message>>
    where
        F: FnOnce() + Send + 'static,
    {
        let message = Message::NewJob(Box::new(job));
        self.sender.send(message)
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 1) Tell all workers to terminate.
        for _ in &self.workers {
            // We ignore errors here: if send fails, workers are already gone.
            let _ = self.sender.send(Message::Terminate);
        }

        // 2) Join each thread, waiting for it to finish.
        for worker in &mut self.workers {
            if let Some(handle) = worker.thread.take() {
                if let Err(err) = handle.join() {
                    eprintln!("Worker {} panicked: {:?}", worker.id, err);
                }
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    /// Spawn a new worker thread looping on the shared receiver.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = {
                    // Lock the receiver to get the next message
                    let lock = receiver.lock().expect("Worker receiver lock poisoned");
                    lock.recv()
                };

                match message {
                    Ok(Message::NewJob(job)) => {
                        // Execute the job
                        job();
                    }
                    Ok(Message::Terminate) => {
                        // Graceful shutdown
                        break;
                    }
                    Err(_) => {
                        // Channel has closed; nothing more to do
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
