use std::thread;
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Condvar, Mutex,
    },
    time::Duration,
};

struct Semaphore {
    permits: Arc<AtomicUsize>,
    condvar: Condvar,
    mutex: Mutex<()>,
}

impl Semaphore {
    fn new(permits: usize) -> Arc<Self> {
        Arc::new(Semaphore {
            permits: Arc::new(AtomicUsize::new(permits)),
            condvar: Condvar::new(),
            mutex: Mutex::new(()),
        })
    }

    fn default() -> Arc<Self> {
        // Uses the number of CPU cores to set the limit of permits to give
        Arc::new(Semaphore {
            permits: Arc::new(AtomicUsize::new(num_cpus::get())),
            condvar: Condvar::new(),
            mutex: Mutex::new(()),
        })
    }

    #[allow(unused_must_use)]
    fn acquire(self: &Arc<Self>) -> Permit {
        loop {
            if self.permits.load(Ordering::Acquire) != 0 {
                self.permits.fetch_sub(1, Ordering::AcqRel);
                return Permit {
                    semaphore: Arc::clone(self),
                };
            }
            let guard = self.mutex.lock().unwrap();
            self.condvar.wait(guard).unwrap();
        }
    }

    fn try_acquire(self: &Arc<Self>) -> Option<Permit> {
        if self.permits.load(Ordering::Acquire) != 0 {
            self.permits.fetch_sub(1, Ordering::AcqRel);
            return Some(Permit {
                semaphore: Arc::clone(self),
            });
        }
        None
    }

    fn release(&self) {
        self.permits.fetch_add(1, Ordering::Release);
        self.condvar.notify_one();
    }
}

struct Permit {
    semaphore: Arc<Semaphore>,
}

impl Drop for Permit {
    fn drop(&mut self) {
        self.semaphore.release();
    }
}

fn main() {
    let semaphore = Semaphore::new(2);

    for _ in 0..5 {
        let semaphore = Arc::clone(&semaphore);
        thread::spawn(move || {
            let permit = semaphore.acquire();
            thread::sleep(Duration::from_millis(500));
            drop(permit);
        });
    }
    thread::sleep(Duration::from_millis(3000));

    let semaphore = Semaphore::default(); // 8
    for _ in 0..10 {
        let semaphore = Arc::clone(&semaphore);
        thread::spawn(move || {
            if let Some(permit) = semaphore.try_acquire() {
                thread::sleep(Duration::from_millis(500));
                drop(permit);
            } else {
                println!("Too many permits given, exiting the thread"); // Will be printed twice
            }
        });
    }
    thread::sleep(Duration::from_millis(4500));
}
