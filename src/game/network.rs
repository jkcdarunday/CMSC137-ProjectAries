use std::net::{UdpSocket, TcpStream};
use std::sync::mpsc::{Sender, Receiver};
use std::io::{Read, Write};

use bincode::serde::*;
use bincode::SizeLimit;

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Move(i16, i16, i16),
    Life(i16, f32),
    Face(i16, i16, i16),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandList {
    pub commands: Vec<Command>,
}

pub fn start_udp(address: &str, server: &str, tx: Sender<CommandList>) {
    let udp = UdpSocket::bind(address).unwrap();
    let mut buf = [0u8; 1024];
    udp.connect(server).unwrap();
    udp.send(b"Hello\n").unwrap();
    loop {
        udp.recv(&mut buf).unwrap();
        // println!("{:?}", self.buf);
        let cs: CommandList = deserialize(&buf).unwrap();
        println!("UDP: {:?}", cs);
        tx.send(cs).unwrap();
    }
}
pub fn start_tcp_receiver(tcp: &mut TcpStream, tx: Sender<CommandList>) {
    let mut buf = [0u8; 1024];
    loop {
        tcp.read(&mut buf).unwrap();
        // println!("{:?}", self.buf);
        let cs: CommandList = deserialize(&buf).unwrap();
        println!("TCP: {:?}", cs);
        tx.send(cs).unwrap();
    }
}
pub fn start_tcp_sender(tcp: &mut TcpStream, rx: Receiver<CommandList>) {
    loop {
        let cs = rx.recv().unwrap();
        let buf: Vec<u8> = serialize(&cs, SizeLimit::Bounded(1024)).unwrap();
        println!("TCP Write: {:?}", cs);
        tcp.write(&buf).unwrap();
    }
}
