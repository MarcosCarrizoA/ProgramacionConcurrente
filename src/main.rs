use std::io::{Read, Write};
use std::net::TcpListener;

fn calculate_pi(n: u64) -> f64 {
    let mut acc = 0.0;
    let mut sign = 1.0;
    for i in 0..n {
        acc += sign / (2 * i + 1) as f64;
        sign = -sign; // Alternar el signo sin usar powi() porque se pierden valores al pasar de u64 a i32
    }
    4.0 * acc
}

fn handle(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap_or(0);

    if bytes_read == 0 {
        return;
    }

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    println!("Request: {}", request);

    let mut lines = request.lines();
    let request_line = lines.next().unwrap_or("");
    let parts: Vec<&str> = request_line.split_whitespace().collect();

    if parts.len() < 3 {
        return;
    }

    let method = parts[0];
    let path = parts[1];

    if method != "GET" {
        let response = "HTTP/1.1 405 METHOD NOT ALLOWED\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
        return;
    }

    if !path.starts_with("/pi/") {
        let response = "HTTP/1.1 404 NOT FOUND\r\n\r\nRuta no encontrada";
        stream.write_all(response.as_bytes()).unwrap();
        return;
    }

    let term_str = &path[4..];
    let n: u64 = match term_str.parse() {
        Ok(num) => num,
        Err(_) => {
            let response = "HTTP/1.1 400 BAD REQUEST\r\n\r\nNúmero inválido";
            stream.write_all(response.as_bytes()).unwrap();
            return;
        }
    };

    let pi = calculate_pi(n);
    let body = format!("Valor de Pi para el término {}: {}", n, pi);
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
        body.len(),
        body
    );

    stream.write_all(response.as_bytes()).unwrap();
}

fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:3030")
        .expect("No se pudo iniciar el servidor en 127.0.0.1:3030");

    println!("Servidor escuchando en 127.0.0.1:3030");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle(stream);
            }
            Err(e) => {
                eprintln!("Error al aceptar conexión: {}", e);
            }
        }
    }
}

fn main() {
    start_server();
}
