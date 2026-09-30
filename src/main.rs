use {std::env::args, std::net::TcpStream, std::net::SocketAddr, std::time::Duration};

fn main() {
    
    let args: Vec<String> = args().collect();

    let s_port: u32 = args[1].parse::<u32>().unwrap();
    let e_port: u32 = args[2].parse::<u32>().unwrap();
    let ip_addr: &String = &args[3];

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