use super::basic::*;
use mio::event::Evented;
use mio::{
    PollOpt,
    Ready,
    Token,
    Events,
    Poll
};
use std::borrow::Borrow;
use std::time::Duration;
use std::io::Result;
use mio::net::{TcpStream, TcpListener};
use std::net::{SocketAddr, ToSocketAddrs, Ipv4Addr, IpAddr};

const READ: Token = Token(0);
const UPDATE: Token = Token(1);
const DELETE: Token = Token(2);
const CREATE: Token = Token(3);

pub trait StorageService {
    async fn send_query(&mut self, event: ActorEvent) -> Result<()>;
}

pub struct DefaultStorageService {
    event_poll: Box<Poll>,
    io: Option<Events>,
    timeout: u64,
    endpoint: Box<dyn ToSocketAddrs>,
    buf_size: usize,
    connector: Box<TcpStream>,
}


impl DefaultStorageService {

    pub fn new(
        event_capacity: &usize,
        timeout: &u64,
        event_buf_size: &usize,
        endpoint: (Ipv4Addr, u16)
    ) -> Self {
        let ref addr = SocketAddr::new(IpAddr::V4(endpoint.0), endpoint.1);
        DefaultStorageService {
            event_poll: Box::new(Poll::new()?),
            io: Some(Events::with_capacity(event_capacity.clone())),
            timeout: timeout.clone(),
            endpoint: Box::new(addr.clone()),
            buf_size: event_buf_size.clone(),
            connector: Box::new(Self::connect(addr, timeout, event_buf_size)),
        }
    }


    fn register(&self, poll: &Poll, opts: PollOpt) {
        let interest = Ready::readable() | Ready::writable();
        let token = READ | CREATE | DELETE | UPDATE;
        self.event_poll.register(poll, token, interest, opts);
    }

    fn handle_events(&mut self, events: &mut Events) {
        self.event_poll.poll(events, Some(Duration::from_millis(self.timeout.clone())))?;
        events.iter().for_each(|x| {
            match x.token() {
                READ => {
                    if x.readiness().is_writable() {
                    }
                }
            }
        })
    }

    pub fn start(&self) {
        self.register(self.event_poll.as_ref(), PollOpt::edge());
        loop {
        }
    }

    fn connect(endpoint: &dyn ToSocketAddrs, timeout: &u64, buf_size: &usize) -> TcpStream {

        let client = TcpStream::connect(endpoint);
        match client {
            Ok(c) => {
                Self::set_connection_properties(c, timeout, buf_size);
            },
            Err(e) => panic!("Storage Engine connection error happens!: {}", e),
        }
        client?
    }

    fn set_connection_properties(c: TcpStream, timeout: &u64, buf_size: &usize) {
        c.set_keepalive(Some(Duration::from_secs(timeout.clone())));
        c.set_send_buffer_size(buf_size.clone());
        c.set_recv_buffer_size(buf_size.clone());
    }
}

impl StorageService for DefaultStorageService {
    async fn send_query(&mut self, actor_event: ActorEvent) -> Result<()> {
    }
}