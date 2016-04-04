use std::net::UdpSocket;
use std::sync::mpsc::Sender;

use bincode::serde::*;

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

pub struct Client{
	con: UdpSocket,
	buf: [u8; 1024]
}

impl Client{
	pub fn new(address: &str) -> Client{
		Client{
			con: UdpSocket::bind(address).unwrap(),
			buf: [0u8; 1024]//Vec::with_capacity(1024)
		}
	}

	pub fn start(&mut self, server: &str, tx: Sender<CommandList>){
		self.con.connect(server).unwrap();
		self.con.send(b"Hello\n").unwrap();
		loop{
			self.con.recv(&mut self.buf).unwrap();
			//println!("{:?}", self.buf);
			let cs: CommandList = deserialize(&self.buf).unwrap();
			println!("{:?}", cs);
			tx.send(cs).unwrap();
		};
	}
}
