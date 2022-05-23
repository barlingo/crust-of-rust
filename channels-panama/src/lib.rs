use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let was_last = inner.senders == 0;
        drop(inner);
        if was_last {
            self.shared.available.notify_one();
        }
    }
}
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);
        Sender {
            shared: Arc::clone(&self.shared),
        }
    }
}
impl<T> Sender<T> {
    pub fn send(&mut self, t: T) -> Result<(), String> {
        let mut inner = self.shared.inner.lock().unwrap();
        if inner.receiver {
            inner.queue.push_back(t);
            drop(inner);
            self.shared.available.notify_one();
            Ok(())
        } else {
            Err(String::from("Receiver was closed unexpectedly"))
        }
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(t) => return Some(t),
                None if inner.senders == 0 => return None,
                None => {
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}
impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.receiver = false;
        drop(inner);
        self.shared.available.notify_all();
    }
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}
struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
    receiver: bool,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: VecDeque::default(),
        senders: 1,
        receiver: true,
    };
    let shared = Shared {
        inner: Mutex::new(inner),
        available: Condvar::new(),
    };
    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared: shared.clone(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(42).unwrap();
        assert_eq!(rx.recv(), Some(42));
    }
    #[test]
    fn closed_tx() {
        let (tx, mut rx) = channel::<()>();
        drop(tx);
        let _ = rx.recv();
    }
    #[test]
    fn closed_rx() {
        let (mut tx, rx) = channel();
        drop(rx);

        match tx.send(42) {
            Ok(_) => {}
            Err(s) => {
                assert_eq!(s, "Receiver was closed unexpectedly");
            }
        }
    }
}
