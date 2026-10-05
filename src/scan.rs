use std::{
    net::{SocketAddr, TcpStream},
    time::Duration,
};

use crate::service;

pub fn scan(s_port: u32, e_port: u32, ip_addr: &str) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();

    for port in s_port..=e_port {
        let addr = format!("{}:{}", ip_addr, port);

        match addr.parse::<SocketAddr>() {
            Ok(addr) => match TcpStream::connect_timeout(&addr, Duration::from_millis(500)) {
                Ok(stream) => {
                    let service = service::detect(stream, port);

                    results.push(format!(
                        "\x1b[32m[OPEN]\x1b[0m \x1b[36m{}\x1b[0m -> \x1b[33m{}\x1b[0m",
                        port, service
                    ));
                }

                Err(_) => {}
            },

            Err(e) => {
                eprintln!("\x1b[31mInvalid address: {}\x1b[0m", e);
            }
        }
    }

    results
}
