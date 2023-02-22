pub mod grpc;
mod greeter;
pub mod tcp;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::net::{UdpSocket};

fn gen_server_id() -> String {
    let rng = thread_rng();
    let s: String = rng.sample_iter(&Alphanumeric)
        .take(32)
        .map(|c: u8| c as char)
        .collect();

    s.to_lowercase()
}


fn get_local_addr() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None
    };

    match socket.connect("8.8.8.8:80") {
        Ok(()) => (),
        Err(_) => return None
    };

    return match socket.local_addr() {
        Ok(s) => Some(s.ip().to_string()),
        Err(_) => None
    };
}