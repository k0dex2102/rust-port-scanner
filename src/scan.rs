use {std::net::TcpStream, std::net::SocketAddr, std::time::Duration};

pub fn scan(s_port: u32, e_port: u32, ip_addr: &String) -> Vec<String> {
    // make a list to save the open ports to
    let mut results:Vec<String> = Vec::new();

    // loop through every single port 
    for port in s_port..=e_port {

        // format every port into and ip address with the port 
        let addr = format!("{}:{}", ip_addr, port);

        // parse the formated address to an SocketAddr
        match addr.parse::<SocketAddr>() {
            Ok(addr) => {

                // connect to ip address with every port and check if its open or not
                match TcpStream::connect_timeout(&addr, Duration::from_millis(500)) {

                    // save the open ports to the list as formated String
                    Ok(_) => results.push(format!("[OPEN] {}", port)),
                    Err(_) => {}

                }
                
            } 
            Err(e) => eprintln!("Invalid address: {e}"),
        }
    }
    // return the open ports
    results
}