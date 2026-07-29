//use std::thread;
//use std::{sync::mpsc, thread};
use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};
/*pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}*/
/*
pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}*/
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
//struct Job;
type Job =Box<dyn FnOnce() +Send + 'static>;

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
        let (sender, receiver) = mpsc::channel();
         let receiver = Arc::new(Mutex::new(receiver));
         let mut workers = Vec::with_capacity(size);
        for id in 0..size {
          // workers.push(Worker::new(id, receiver));
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }
    
   
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

          self.sender.as_ref().unwrap().send(job).unwrap();
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
                let message = receiver.lock().unwrap().recv();
                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");

                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker { id, thread }
    }
}
// Sending Requests to Threads via Channels=======

/*The ThreadPool will create a channel and hold on to the sender.
Each Worker will hold on to the receiver.
We’ll create a new Job struct that will hold the closures we want to send down the channel.
The execute method will send the job it wants to execute through the sender.
In its thread, the Worker will loop over its receiver and execute the closures of any jobs it receives.*/


//Implementing the execute Method========

//Graceful shutdown and cleanup ====== 
//Implementing the Drop Trait on ThreadPool
/*
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            worker.thread.join().unwrap();
        }
    }
}*///error prone

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);
            worker.thread.join().unwrap();
        }
    }
}

//Signaling to the Threads to Stop Listening for Jobs



