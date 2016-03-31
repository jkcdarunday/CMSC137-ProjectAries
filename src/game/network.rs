use std::net::UdpSocket;
use std::thread;
use std::sync::mpsc::Sender;
use std::net::ToSocketAddrs;

pub struct Network{
	con: UdpSocket,
	buf: [u8;1024]
}

pub enum Command{
	Move(i16, i16, i16),
	Life(i16, f32),
	Face(i16, i16, i16),
}

impl Network{
	fn new(address: &str) -> Network{
		Network{
			con: UdpSocket::bind(address).unwrap(),
			buf: [0u8; 1024]
		}
	}

	fn start(&mut self, server: &str, tx: Sender<Vec<(u16, u16)>>){
		self.con.connect(server);
		loop{
			self.con.recv(&mut self.buf).unwrap();
		};
	}
}
