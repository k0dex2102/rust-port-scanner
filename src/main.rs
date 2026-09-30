use {std::env::args};
mod scan;

fn main() {
    
    // make args string vector
    let args: Vec<String> = args().collect();
    
    // error checking if all parameters are provided
    if args.len() < 4 {
        println!("usage: <start port> <end port> <IP addr>");
        return;
    }

    // port and ip parameters
    let s_port: u32 = args[1].parse::<u32>().unwrap();
    let e_port: u32 = args[2].parse::<u32>().unwrap();
    let ip_addr: &String = &args[3];

    // check if ports are not above max port 
    if s_port >= 65536 || e_port >= 65536 {
        println!("Max port range!");
        return;
    // checking if start port is higher then end port
    } else if s_port > e_port {
        println!("Start port must be higher then end port!");
        return;
    }

    scan::scan(s_port, e_port, ip_addr);
}