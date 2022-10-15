use std::net::{UdpSocket, TcpListener, TcpStream};
use std::str;
use Vec;
use evmap;
use std::io::{self, Write};


use crate::bike::Bike;

fn handle_tcp(mut stream: TcpStream) {
    stream.write(b"HTTP/1.0 200 OK
Content-type: text/html
Content-Length: 13

presence=true\0");
}


pub struct Receiver {
    socket: UdpSocket,
    tcp_socket: TcpListener,
    pub reader: evmap::ReadHandle<String, Bike>,
    writer: evmap::WriteHandle<String, Bike>,

}

impl Receiver {
    pub fn new(port: &str) -> Receiver {
        let mut bind_addr: String = String::from("0.0.0.0:");
        bind_addr.push_str(port);

        let socket = UdpSocket::bind(&bind_addr).expect("Failed to bind to address");
        let mut tcp_socket = TcpListener::bind(&bind_addr).expect("Failed to bind tcp to address");
        let (bikes_r, mut bikes_w) = evmap::new();

        Receiver {
            socket: socket,
            tcp_socket: tcp_socket,
            reader: bikes_r,
            writer: bikes_w,
        }
    }

    pub fn receiver_loop(&mut self) {
        let mut buffer: [u8; 256] = [0;256];
        self.socket.set_nonblocking(true).unwrap();
        
        loop {
            
            for stream in self.tcp_socket.incoming() {
                handle_tcp(stream.expect("msg"));
            }

            let filled_buffer: Vec<u8>;
            /*match self.socket.recv_from(&mut buffer) {
                Ok(n) => {filled_buffer = buffer[..n.0].to_vec();},
                Err(_n) => (continue)
             }*/
             //if number_of_bytes == 0 {continue;}
             //self.parse_packet(filled_buffer);

             let (num_bytes_read, _) = loop {
                match self.socket.recv_from(&mut buffer) {
                    Ok(n) => break n,
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        // wait until network socket is ready, typically implemented
                        // via platform-specific APIs such as epoll or IOCP
                        //wait_for_fd();
                        println!("Didnt work");
                    }
                    Err(e) => panic!("encountered IO error: {e}"),
        
                }
            };


        }
    }

    pub fn get_bikes(&mut self) -> Vec<Bike> {
        let mut ret = Vec::new();
        for (_id, bikes) in &self.reader.read().unwrap() {
            // Should only be one bike
            for bike in bikes {
                ret.push(bike.clone());
            }
        }
        return ret
    }


    // BIKE PARSING FUNCTIONS
    fn parse_packet(&mut self, buffer: Vec<u8>) {
        //packet should be 49 in length

        //Packets are 49 in length, just drop if it doesn't match
        if buffer.len() != 49 {
            return
        }

        //Check for the magic bytes that define the packet
        let magic = ((buffer[1] as u16) << 8) + buffer[0] as u16;
        if magic != 0x3114 { // or 0x1431 not sure
            return
        }
        let id = String::from(str::from_utf8(&buffer[4..21]).expect("Could not parse bytes to id")); 
        let watt = ((buffer[19] as u16) << 8) + buffer[18] as u16; 
        let watt_percentage = buffer[14]; 
        let rpm = buffer[14]; // TODO: Find den rigtige RPM, det er muligt den er lige efter, men mine noter er ikke så gode

        //Check if id is in array
        /*if !true {
            //The ID doesn't exist, so we add it
            let bike = Bike::new(id.clone());
            self.writer.insert(id.clone(), bike);
            self.writer.refresh();
            //self.bikes.lock().unwrap().insert(id.clone(),bike);
        }*/

        //Add the values to the right bike
        self.writer.empty(id.clone());
        let new_bike = Bike::new_val(id.clone(), watt, watt_percentage, rpm);
        self.writer.insert(id, new_bike);
        self.writer.refresh();

        /*self.bikes.lock().unwrap().get(&id).unwrap().update(
            watt,
            watt_percentage,
            rpm
        );*/


    }

    
}