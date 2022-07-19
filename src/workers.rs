#![allow(unused)]

use std::{thread::{JoinHandle, self}, sync::mpsc::Sender};

pub struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    
}

pub struct WorkerPool {
    workers: Vec<Worker>,
    sender: Sender<Box<dyn FnOnce() + Send + 'static>>,
}
