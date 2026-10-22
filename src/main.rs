use std::{env::args, time::Instant};

// push test

mod scan;
mod service;

fn main() {
    let args: Vec<String> = args().collect();

    let start = Instant::now();

    // Error checking if all parameters are provided
    if args.len() < 4 {
        println!("usage: <start port> <end port> <IP addr>");
        return;
    }

    let s_port: u32 = match args[1].parse() {
        Ok(port) => port,
        Err(_) => {
            println!("Invalid start port!");
            return;
        }
    };

    let e_port: u32 = match args[2].parse() {
        Ok(port) => port,
        Err(_) => {
            println!("Invalid end port!");
            return;
        }
    };

    let ip_addr: &str = &args[3];

    // Check if ports are not above max port
    if s_port > 65535 || e_port > 65535 {
        println!("Max port range!");
        return;
    }

    // Check if start port is higher than end port
    if s_port > e_port {
        println!("Start port must not be higher than end port!");
        return;
    }

    let results = scan::scan(s_port, e_port, ip_addr);

    // Save elapsed time and print it
    let duration = start.elapsed();

    println!("Time: {:.2?}", duration);

    // Print open ports
    for result in results {
        println!("{}", result);
    }
}
