#![feature(custom_derive, plugin, sockaddr_setters)]
#![plugin(serde_macros)]

extern crate serde;
extern crate bincode;

use bincode::serde::*;

use std::net::{UdpSocket,TcpListener, TcpStream};
use std::thread;

mod game{
	pub mod network;
}

use game::network::{Command, CommandList};

fn handle_client(stream: TcpStream) {
    // ...
}

fn main() {
    let mut usocket: UdpSocket = UdpSocket::bind("0.0.0.0:6665").unwrap();
	let mut tsocket: TcpListener = TcpListener::bind("0.0.0.0:6660").unwrap();



	for stream in tsocket.incoming() {
	    match stream {
	        Ok(stream) => {

	            thread::spawn(move|| {
	                // connection succeeded
	                handle_client(stream)
	            });
	        }
	        Err(e) => { /* connection failed */ }
	    }
	}

    /*
	let mut buf = [0u8; 1024];
    let (size, addr) = usocket.recv_from(&mut buf).unwrap();

	let c1 = CommandList { commands: vec![Command::Move(0, 5, 5)] };
    let c1s = serialize(&c1, bincode::SizeLimit::Bounded(1024)).unwrap();

    let c2 = CommandList { commands: vec![Command::Move(0, 20, 20)] };
    let c2s = serialize(&c2, bincode::SizeLimit::Bounded(1024)).unwrap();

    println!("Connection received: {:?}", addr);

    let mut t = false;
    while true {
        if t {
            socket.send_to(&c1s, addr);
        } else {
            socket.send_to(&c2s, addr);
        }

        t = !t;

        std::thread::sleep_ms(1000);
    }*/
}
