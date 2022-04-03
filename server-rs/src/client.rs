use log::{debug, warn};
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;

pub struct Client {
    stream: TcpStream,
    handle: Option<JoinHandle<io::Result<()>>>,
    running: Arc<AtomicBool>,
}


impl Client {
    pub fn new(stream: TcpStream) -> Self {
        let handle = None;
        let running = Arc::new(AtomicBool::new(false));
        Self {
            stream,
            handle,
            running,
        }
    }

    /// launches a thread that reads lines from tcp socket
    pub fn start(&mut self) {
        if let Ok(mut stream) = self.stream.try_clone() {
            let running_flag = self.running.clone();
            running_flag.store(true, Ordering::Relaxed);
            let handle = std::thread::spawn(move || {
                let read_stream = stream.try_clone().unwrap();
                let reader = BufReader::new(read_stream);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        debug!("Message arrived: {:?}", line);
                        stream.write(line.as_bytes())?;
                        stream.write(&['\n' as u8])?;
                    } else {
                        warn!("Invalid message arrived! closing client");
                        break;
                    }
                }
                running_flag.store(false, Ordering::Relaxed);
                Ok(())
            });
            self.handle = Some(handle);
        }
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    /// closes socket and flags client as not running
    pub fn stop(&mut self) {
        self.stream.shutdown(Shutdown::Both);
        self.running.store(false, Ordering::Relaxed)
    }

    /// joins thread handler
    pub fn join(&mut self) {
        if let Some(mut handle) = self.handle.take() {
            handle.join();
        }
    }
}
