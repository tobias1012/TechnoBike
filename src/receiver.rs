use std::net::{UdpSocket};
use std::str;
use Vec;
use evmap;

use crate::bike::Bike;

pub struct Receiver {
    socket: UdpSocket,
    reader: evmap::ReadHandle<String, Bike>,
    writer: evmap::WriteHandle<String, Bike>,

}

impl Receiver {
    pub fn new(port: &str) -> Receiver {
        let mut bind_addr: String = String::from("127.0.0.1:");
        bind_addr.push_str(port);

        let socket = UdpSocket::bind(bind_addr).expect("Failed to bind to address");
        let (bikes_r, mut bikes_w) = evmap::new();

        Receiver {
            socket: socket,
            reader: bikes_r,
            writer: bikes_w,
        }
    }

    pub fn receiver_loop(&mut self) {
        let mut buffer: [u8; 256] = [0;256];
        

        
        loop {
             let (number_of_bytes, src_addr) = self.socket.recv_from(&mut buffer).expect("Didn't recieve data");
            
             let filled_buffer = buffer[..number_of_bytes].to_vec();
             self.parse_packet(filled_buffer);
        }
    }

    pub fn get_bikes(&mut self) -> Vec<Bike> {
        let mut ret = Vec::new();
        for (id, bikes) in &self.reader.read().unwrap() {
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
        if magic != 0x3114 {
            return
        }
        let id = String::from(str::from_utf8(&buffer[10..14]).expect("Could not parse bytes to id")); // ARBITRARY VALUES TODO: Find the real offsets
        let watt = ((buffer[1] as u16) << 8) + buffer[0] as u16; // ARBITRARY VALUES TODO: Find the real offsets
        let watt_percentage = buffer[1]; // ARBITRARY VALUES TODO: Find the real offsets
        let rpm = ((buffer[1] as u16) << 8) + buffer[0] as u16; // ARBITRARY VALUES TODO: Find the real offsets

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