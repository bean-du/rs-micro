use std::collections::HashMap;
use std::fs::read;
use std::io;
use std::sync::mpsc::Receiver;
use std::task::ready;
use prost::bytes::BufMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt, Interest};
use tokio::net::TcpStream;
use crate::registry::IResult;
use tokio::sync::mpsc;

pub struct TcpServer {
    pub addr: String,
    pub conns: HashMap<String, TcpStream>,
}


pub struct Conn {
    pub id: String,
    pub tx: mpsc::Sender<Vec<u8>>,
    pub rx: mpsc::Receiver<Vec<u8>>,
}

impl Conn {
    pub fn new(id: String, tx: mpsc::Sender<Vec<u8>>, rx: mpsc::Receiver<Vec<u8>>) -> Self {
        Self {
            id,
            tx,
            rx,
        }
    }
}

impl TcpServer {
    pub fn new(addr: String) -> Self {
        let conns = HashMap::new();
        Self { addr, conns }
    }

    pub async fn start(&self) -> IResult<()> {
        let listener = tokio::net::TcpListener::bind(&self.addr).await?;

        loop {
            let (socket, _) = listener.accept().await?;

            self.handle_conn(socket);
        }
    }

    pub fn handle_conn(&self, socket: TcpStream) {
        let (tx, mut rx): (mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>) = mpsc::channel(100);
        let (reader, writer) = socket.into_split();

        tokio::spawn(async move {
            let mut i = 1;
            loop {
                let reade_ready = match reader.ready(Interest::READABLE).await {
                    Ok(r) => r,
                    Err(e) => {
                        println!("socket listen event error: {}", e);
                        return;
                    }
                };
                println!("{}", i);
                if reade_ready.is_readable() {
                    let mut data = vec![0; 1024];
                    // Try to read data, this may still fail with `WouldBlock`
                    // if the readiness event is a false positive.
                    match reader.try_read(&mut data) {
                        Ok(n) => {
                            println!("read {} bytes", n);
                            println!("data {:?} ", String::from_utf8(data.clone()));
                            // 读取到数据后，这里调用具体的处理函数处理
                            match tx.send(data.to_vec()).await {
                                Ok(_) => { println!("send data to channel success") }
                                Err(e) => { println!("send data to channel failed: {}", e) }
                            }
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            println!("read data error {}", e);
                            continue;
                        }
                        Err(e) => {
                            println!("read data error {}", e);
                            return;
                        }
                    }
                }
                i +=1;
            }
        });

        tokio::spawn(async move {
            let write_ready = match writer.ready(Interest::WRITABLE).await {
                Ok(r) => r,
                Err(e) => {
                    println!("socket listen event error: {}", e);
                    return;
                }
            };

            loop {
                let data = match rx.recv().await {
                    Some(d) => d,
                    None => { continue; }
                };

                println!("Received: {:?}", data);

                if write_ready.is_writable() {
                    // Try to write data, this may still fail with `WouldBlock`
                    // if the readiness event is a false positive.
                    match writer.try_write(&data) {
                        Ok(n) => {
                            println!("write {} bytes", n);
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            println!("write data error {}", e);
                            continue;
                        }
                        Err(e) => {
                            println!("write data error {}", e);
                            return;
                        }
                    }
                }
            }
        });
    }
}

