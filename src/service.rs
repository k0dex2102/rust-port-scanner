use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

pub fn detect(mut stream: TcpStream, port: u32) -> String {
    let _ = stream.set_read_timeout(Some(Duration::from_millis(300)));
    let _ = stream.set_write_timeout(Some(Duration::from_millis(300)));

    let mut buffer = [0u8; 2048];

    if let Ok(size) = stream.read(&mut buffer) {
        if size > 0 {
            let response = String::from_utf8_lossy(&buffer[..size]);

            if response.starts_with("SSH-") {
                return format!("SSH ({})", first_line(&response));
            }

            if response.starts_with("220 ") {
                if response.contains("SMTP") {
                    return format!("SMTP ({})", first_line(&response));
                }

                return format!("FTP ({})", first_line(&response));
            }
        }
    }

    if probe_http(&mut stream, &mut buffer) {
        return parse_http(&buffer);
    }

    match port {
        139 => "NetBIOS/SMB?".to_string(),
        445 => "SMB?".to_string(),
        631 => "IPP/CUPS?".to_string(),

        _ => "unknown".to_string(),
    }
}

fn probe_http(stream: &mut TcpStream, buffer: &mut [u8]) -> bool {
    let request = b"GET / HTTP/1.0\r\nHost: localhost\r\n\r\n";

    if stream.write_all(request).is_err() {
        return false;
    }

    if let Ok(size) = stream.read(buffer) {
        if size > 0 {
            let response = String::from_utf8_lossy(&buffer[..size]);

            return response.starts_with("HTTP/1.");
        }
    }

    false
}

fn parse_http(buffer: &[u8]) -> String {
    let response = String::from_utf8_lossy(buffer);

    let mut server = None;
    let mut status = None;

    for line in response.lines() {
        if line.starts_with("HTTP/") {
            status = Some(line.trim());
        }

        if let Some(value) = line.strip_prefix("Server:") {
            server = Some(value.trim());
        }
    }

    match (status, server) {
        (Some(status), Some(server)) => {
            format!("HTTP -> {} ({})", server, status)
        }

        (Some(status), None) => {
            format!("HTTP ({})", status)
        }

        _ => "HTTP".to_string(),
    }
}

fn first_line(response: &str) -> &str {
    response.lines().next().unwrap_or("unknown")
}
