use std::net::UdpSocket;
use std::thread;
use std::sync::mpsc::Sender;
use std::net::ToSocketAddrs;

use bincode::serde::*;

pub struct Network{
	con: UdpSocket,
	buf: [u8; 1024]
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command{
	Move(i16, i16, i16),
	Life(i16, f32),
	Face(i16, i16, i16),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandList{
	pub commands: Vec<Command>
}

impl Network{
	pub fn new(address: &str) -> Network{
		Network{
			con: UdpSocket::bind(address).unwrap(),
			buf: [0u8; 1024]//Vec::with_capacity(1024)
		}
	}

	pub fn start(&mut self, server: &str, tx: Sender<CommandList>){
		self.con.connect(server);
		self.con.send(b"Hello\n");
		loop{
			self.con.recv(&mut self.buf).unwrap();
			//println!("{:?}", self.buf);
			let cs: CommandList = deserialize(&self.buf).unwrap();
			println!("{:?}", cs);
			tx.send(cs);
		};
	}
}
