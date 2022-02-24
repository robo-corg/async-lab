use anyhow::Result;
use std::collections::HashMap;
use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::task;

struct Executor {
    poll: mio::Poll,
    events: mio::Events,
    next_token: AtomicUsize,
    wakers: HashMap<mio::Token, std::task::Waker>
}

impl Executor {
    fn create() -> Result<Self> {
        Ok(Executor {
            poll: mio::Poll::new()?,
            events: mio::Events::with_capacity(128),
            next_token: AtomicUsize::new(0),
            wakers: HashMap::new()
        })
    }

    fn create_token(&self) -> mio::Token {
        mio::Token(self.next_token.fetch_add(1, Ordering::SeqCst))
    }

    fn listen_socket(&self, addr: &str) -> Result<TcpListener> {
        let addr = addr.parse()?;
        let mut server = mio::net::TcpListener::bind(addr)?;

        let token = self.create_token();

        Ok(TcpListener {
            token,
            listener: server,
        })
    }

    fn add_waker(&self, token: mio::Token, waker: std::task::Waker) {

    }
}

struct TcpListener {
    token: mio::Token,
    listener: mio::net::TcpListener,
}

impl TcpListener {
    fn accept<'a>(&'a mut self, executor: &'a Executor) -> TcpAcceptFuture<'a> {
        TcpAcceptFuture {
            executor,
            listener: self,
        }
    }
}

struct TcpAcceptFuture<'a> {
    executor: &'a Executor,
    listener: &'a mut TcpListener,
}

impl<'a> Future for TcpAcceptFuture<'a> {
    type Output = Result<(mio::net::TcpStream, SocketAddr)>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match self.listener.listener.accept() {
            Ok(ret) => task::Poll::Ready(Ok(ret)),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                let fut_self = self.get_mut();

                fut_self.executor.add_waker(fut_self.listener.token, waker);

                // Start listening for incoming connections.
                fut_self.executor.poll.registry().register(
                    &mut fut_self.listener.listener,
                    fut_self.listener.token.clone(),
                    mio::Interest::READABLE,
                )?;

                task::Poll::Pending
            }
            Err(e) => task::Poll::Ready(Err(e.into())),
        }
    }
}

fn main() -> Result<()> {
    let mut executor = Executor::create()?;

    Ok(())
}
