use std::{
    net::{SocketAddr, TcpStream},
    time::Duration,
};

pub fn scan(s_port: u32, e_port: u32, ip_addr: &String) -> Vec<String> {
    // make a list to save the open ports to
    let mut results: Vec<String> = Vec::new();

    for port in s_port..=e_port {
        let addr = format!("{}:{}", ip_addr, port);

        // parse the formatted address to a SocketAddr
        match addr.parse::<SocketAddr>() {
            Ok(addr) => {
                // connect to the IP address and check if the port is open
                match TcpStream::connect_timeout(&addr, Duration::from_millis(500)) {
                    // save the open ports to the list as formatted String
                    Ok(_) => results.push(format!("[OPEN] {}", port)),
                    Err(_) => {}
                }
            }
            Err(e) => eprintln!("Invalid address: {e}"),
        }
    }
    results
}