use crate::client::Client;
use log::{debug, warn};
use std::io;
use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Object that represents a TCP connection acceptor
pub struct Server {
    listener: TcpListener,
}

impl Server {
    /// Binds listener to the address `addr`
    pub fn new(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;
        Ok(Self { listener })
    }

    /// Starts server, hooking to signals `SIGTERM` and `SIGINT`
    /// Listens to incoming connections until one of these signals is triggered
    /// Launchs a thread per client instance
    pub fn run(&mut self) -> io::Result<()> {
        let term = Arc::new(AtomicBool::new(false));
        signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))?;
        signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term))?;
        let mut clients = vec![];
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let mut client = Client::new(stream);
                    client.start();
                    clients.push(client);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_secs(1));
                }
                Err(e) => panic!("encountered IO error: {}", e),
            }
            if term.load(Ordering::Relaxed) {
                warn!("Server going down!");
                break;
            }
            // into_iter consumes vector
            let alive: Vec<_> = clients
                .into_iter()
                .filter_map(|mut client| {
                    if client.is_running() {
                        Some(client)
                    } else {
                        debug!("Joining client");
                        client.join();
                        None
                    }
                })
                .collect();
            clients = alive;
        }
        clients.iter_mut().for_each(|client| {
            debug!("Joining client");
            client.stop();
            client.join();
        });
        Ok(())
    }
}
