use {std::net::TcpStream, std::net::SocketAddr, std::time::Duration};

pub fn scan(s_port: u32, e_port: u32, ip_addr: &String) {
    for port in s_port..=e_port {
        let addr = format!("{}:{}", ip_addr, port);
        match addr.parse::<SocketAddr>() {
            Ok(addr) => {
                match TcpStream::connect_timeout(&addr, Duration::from_millis(500)) {
                    Ok(_) => println!("[OPEN] {}", port),
                    Err(_) => {}
                }
            } 
            Err(e) => eprintln!("Invalid address: {e}"),
        }
    }
}