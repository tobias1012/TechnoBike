use std::net::{UdpSocket, TcpListener, TcpStream};
use std::str;
use std::sync::{Mutex, MutexGuard};
use Vec;
use evmap;


use crate::bike::Bike;


pub struct Receiver {
    socket: UdpSocket,
    pub reader: evmap::ReadHandle<String, Bike>,
    //pub writer: evmap::WriteHandle<String, Bike>,
    pub writer: Mutex<evmap::WriteHandle<String, Bike>>

}

impl Receiver {
    pub fn new(port: &str) -> Receiver {
        let mut bind_addr: String = String::from("0.0.0.0:");
        bind_addr.push_str(port);

        let socket = UdpSocket::bind(&bind_addr).expect("Failed to bind to address");
        let (bikes_r, bikes_w) = evmap::new();

        Receiver {
            socket: socket,
            reader: bikes_r,
            writer: Mutex::new(bikes_w),
        }
    }


    pub fn receiver_loop(&mut self) {
        let mut buffer: [u8; 256] = [0;256];
        //self.socket.set_nonblocking(true).unwrap();
        //self.tcp_socket.set_nonblocking(true).unwrap();
        
        loop {

            let filled_buffer: Vec<u8>;
            match self.socket.recv_from(&mut buffer) {
                Ok(n) => {filled_buffer = buffer[..n.0].to_vec();},
                Err(_n) => {println!("failed recieving data"); continue}
             }
             //if number_of_bytes == 0 {continue;}
             self.parse_packet(filled_buffer);

             /*let (num_bytes_read, _) = loop {
                match self.socket.recv_from(&mut buffer) {
                    Ok(n) => break n,
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        // wait until network socket is ready, typically implemented
                        // via platform-specific APIs such as epoll or IOCP
                        //wait_for_fd();
                        println!("Didnt work");
                    }
                    Err(e) => panic!("encountered IO error: {e}"),
        
                }*/


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
            println!("packet too short");
            return
        }

        //Check for the magic bytes that define the packet
        let magic = ((buffer[1] as u16) << 8) + buffer[0] as u16;
        if magic != 0x3114 { // or 0x1431 not sure
            println!("packet not the right magic byte");
            return
        }

        //lock the mutex
        let mut state_guard: MutexGuard<evmap::WriteHandle<String, Bike>> = self.writer.lock().unwrap();


        //println!("parsing data");
        let id = String::from(str::from_utf8(&buffer[5..21]).expect("Could not parse bytes to id")); 
        //let watt = ((buffer[19] as u16) << 8) + buffer[18] as u16; 
        //let watt = buffer[24] as u16;
        let watt = (buffer[24] as u16) + ((buffer[23] as u16) * 255);
        let max_watt = buffer[28] as u16; //tested, might be right
        let watt_percentage = buffer[41]; //TODO: find den rigtige 
        let rpm = buffer[31]; // TODO: Find den rigtige RPM, det er muligt den er lige efter, men mine noter er ikke så gode
    
        //Check if id is in array
        /*if !true {
            //The ID doesn't exist, so we add it
            self.writer.insert(id.clone(), bike);
            self.writer.refresh();
            //self.bikes.lock().unwrap().insert(id.clone(),bike);
        }*/

        //set the name if we cant get it
        let mut name: String = String::from("null");
        let mut age: u8 = 0;
        let mut weight: u16 = 75; 
        if let Some(bikes) = self.reader.get(&id) {
            for bike in &*bikes {
                name = bike.name.clone();
                age = bike.age;
                weight = bike.weight;
            }
        }

        //Add the values to the right bike
        state_guard.empty(id.clone());
        let new_bike = Bike::new_val(id.clone(), watt, watt_percentage, rpm, name, age, weight);
        state_guard.insert(id, new_bike);
        state_guard.refresh();

        /*self.bikes.lock().unwrap().get(&id).unwrap().update(
            watt,
            watt_percentage,
            rpm
        );*/


    }

    
}