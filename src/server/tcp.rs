use std::collections::HashMap;
use std::io;
use tokio::io::{Interest};
use tokio::net::TcpStream;
use tokio::net::unix::SocketAddr;
use crate::registry::IResult;
use tokio::sync::mpsc;
use crate::protocol::packet;
use crate::protocol::packet::{ByteBuffer, Packet};

pub struct TcpServer {
    pub addr: String,
    pub conns: HashMap<String, TcpStream>,
}


pub struct Conn {
    pub id: String,
    pub ip: SocketAddr,
    pub tx: mpsc::Sender<Vec<u8>>,
}

impl Conn {
    pub fn new(id: String, ip: SocketAddr, tx: mpsc::Sender<Vec<u8>>) -> Self {
        Self {
            id,
            ip,
            tx,
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

            match self.handle_conn(socket) {
                Ok(_) => {}
                Err(e) => {}
            }
        }
    }

    pub fn handle_conn(&self, socket: TcpStream) -> IResult<()> {
        let (tx, mut rx): (mpsc::Sender<Packet>, mpsc::Receiver<Packet>) = mpsc::channel(100);
        let (reader, writer) = socket.into_split();

        tokio::spawn(async move {
            loop {
                let reade_ready = match reader.ready(Interest::READABLE).await {
                    Ok(r) => r,
                    Err(e) => {
                        println!("socket listen event error: {}", e);
                        return;
                    }
                };

                if reade_ready.is_readable() {
                    let mut pkt = ByteBuffer::new();
                    // Try to read data, this may still fail with `WouldBlock`
                    // if the readiness event is a false positive.
                    match reader.try_read(&mut pkt.buf) {
                        Ok(n) => {
                            let packets = match pkt.get_packet() {
                                Ok(p) => p,
                                Err(e) => {
                                    println!("parse packet error: {}", e);
                                    continue
                                }
                            };

                            for packet in packets {
                                // 读取到数据后，这里调用具体的处理函数处理
                                println!("Data: {}\n", packet);
                                match tx.send(packet).await {
                                    Ok(_) => {}
                                    Err(e) => { println!("send data to channel failed: {}", e) }
                                }
                            }


                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            continue;
                        }
                        Err(e) => {
                            println!("read data error {}", e);
                            return;
                        }
                    }
                }
            }
        });

        tokio::spawn(async move {
            while let Some(data) = rx.recv().await {
                let write_ready = match writer.ready(Interest::WRITABLE).await {
                    Ok(r) => r,
                    Err(e) => {
                        println!("socket listen event error: {}", e);
                        return;
                    }
                };
                if write_ready.is_writable() {
                    // Try to write data, this may still fail with `WouldBlock`
                    // if the readiness event is a false positive.
                    match writer.try_write(&data.to_string().as_bytes()) {
                        Ok(n) => {}
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

        Ok(())
    }
}

