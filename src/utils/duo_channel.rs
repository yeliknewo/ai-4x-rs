use std::sync::mpsc::{Receiver, Sender, TryRecvError, channel};

pub type BothChannel<T, F> = (DuoChannel<T, F>, DuoChannel<F, T>);

#[derive(Debug)]
pub struct DuoChannel<S, R> {
    send: Sender<S>,
    recv: Receiver<R>,
}

impl<S, R> DuoChannel<S, R> {
    pub fn new_both() -> BothChannel<S, R> {
        let (send_to, recv_to) = channel();
        let (send_from, recv_from) = channel();

        (DuoChannel::new(send_to, recv_from), DuoChannel::new(send_from, recv_to))
    }

    fn new(send: Sender<S>, recv: Receiver<R>) -> DuoChannel<S, R> {
        DuoChannel {
            send: send,
            recv: recv,
        }
    }

    pub fn send(&mut self, event: S) {
        self.send.send(event).unwrap_or_else(|err| panic!("Failed to send because: {}", err));
    }

    pub fn recv(&mut self) -> R {
        self.recv.recv().unwrap_or_else(|err| panic!("Recv from error: {}", err))
    }

    pub fn try_recv(&mut self) -> Option<R> {
        match self.recv.try_recv() {
            Ok(event) => Some(event),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => panic!("Try recv from was disconnected"),
        }
    }
}
