use {std::{env::args, time::Instant}};
mod scan;

fn main() {
    
    // make args string vector and setup runtime
    let args: Vec<String> = args().collect();
    let start = Instant::now();
    
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

    let results = scan::scan(s_port, e_port, ip_addr);

    // save elapsed time into an duration var and print elapsed time for the scan
    let duration = start.elapsed();
    println!("Time: {:.2?}", duration);

    // print the open ports
    for result in results {
        println!("{}", result);
    }

}