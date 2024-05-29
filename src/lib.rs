use std::{num::NonZeroUsize, sync::{mpsc, Condvar, Mutex}};

pub struct MultiThread {
    current_thread: bool,
    senders: Vec<mpsc::Sender<Job<'static>>>,
}

impl MultiThread {
    pub fn new(n_threads: NonZeroUsize) -> MultiThread {
        let n_threads = n_threads.get();
        let mut result = MultiThread {
            current_thread: false,
            senders: Vec::with_capacity(n_threads)
        };
        result
    }

    pub fn new_current_thread() -> MultiThread {
        MultiThread { current_thread: true , senders: Vec::new(), }
    }
}

struct Job<'a> {
    f: &'a (dyn Fn() + Sync),
    _g: JobGuard<'a>,
}

struct  JobCount {
    mux: Mutex<usize>,
    cv: Condvar,
}

// Increments job count on creation and decrements on drop
struct JobGuard<'a> {
    count: &'a JobCount,
}

impl<'a> Job<'a> {
    unsafe fn erase_lifetime(self) -> Job<'static> {
        std::mem::transmute(self)
    }
}

impl JobCount {
    fn new() -> JobCount {
        JobCount { mux: Mutex::new(0), cv: Condvar::new() }
    }

    fn inc(&self) -> JobGuard<'_> {
        *self.mux.lock().unwrap() += 1;
        JobGuard { count: self }
    }

    fn dec(&self) {
        let mut g = self.mux.lock().unwrap();
        *g -= 1;
        if *g == 0 {
            self.cv.notify_all()
        }
    }
}
