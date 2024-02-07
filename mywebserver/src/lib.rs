// use std::ops::Receiver;
use std::sync::{Arc, mpsc};
use std::thread;
use std::thread::JoinHandle;
use std::sync::Mutex;




pub struct ThreadPool {
    workers:Vec<Worker>,
    sender:mpsc::Sender<Job>,
}
type  Job=Box<dyn FnOnce()+Send+'static>;
impl ThreadPool {
    pub fn new(size:usize)->ThreadPool{
        assert!(size>0);
        let mut workers=Vec::with_capacity(size);
        let (sender,receiver)=mpsc::channel();
        let receiver=Arc::new( Mutex::new(receiver));
        for id in 0..size{
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        ThreadPool{workers,sender}
    }
    pub fn  exe<F>(&self,f:F)where
    F:FnOnce()+Send+'static,{
        let job:Box<F>=Box::new(f);
        self.sender.send(job).unwrap();

    }

}
struct Worker{
    id:usize,
    thread:thread::JoinHandle<()>
}

impl Worker {
    fn  new(id:usize, reseiver:Arc<Mutex<mpsc::Receiver<Job>>>) ->Worker{
        let thread: JoinHandle<()>=thread::spawn(move|| loop {
            let job:Box<dyn FnOnce()+Send>=reseiver.lock().unwrap().recv().unwrap();
            println!("==============={}====job:exe",id);
            job();
        });
        Worker{id,thread}
    }
}