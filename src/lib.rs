use std::net::{TcpListener, TcpStream};
use std::io::Write;
use tiny_http::{Server, Response};

pub mod receiver;
pub mod bike;

use receiver::Receiver;

pub fn start_listener() -> receiver::Receiver {

    let mut recv = Receiver::new("1508");

    recv
}


fn handle_tcp(mut stream: TcpStream) {
    match stream.write(b"HTTP/1.0 200 OK
Content-type: text/html
Content-Length: 13

presence=true\0") {
    Ok(_) => {},
    Err(_) => {}
}
}

pub fn http() {
    let server = Server::http("0.0.0.0:1508").unwrap();

    loop {
        // blocks until the next request is received
        let request = match server.recv() {
            Ok(rq) => rq,
            Err(e) => { println!("error: {}", e); continue; }
        };
    
        // do something with the request
        println!("HTTP Request recieved");
        let mut res = Response::from_string("presence=true");
        let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap();
        res.add_header(header);
        let _ = request.respond(res);
    }
}

pub fn tcp_loop() {
    let tcp_socket = TcpListener::bind("0.0.0.0:1508").expect("Failed to bind tcp to address");

    loop {
        for stream in tcp_socket.incoming() {
            println!("GOT HTTP");
            match stream {
                Ok(stream) => {handle_tcp(stream);},
                Err(_) => {break;}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
