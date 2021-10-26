use crate::logger::Logger;
use crate::network::protocol::compression::decompress;
use crate::network::protocol::mcbe::login::deconstruct;
use binary_utils::*;
use mcpe_protocol::interfaces::{Slice, VarSlice};
use mcpe_protocol::mcpe::*;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use mcpe_protocol::mcpe::{GamePacket, construct_packet};
use rakrs::conn::Connection;
use rakrs::raknet_start;
use rakrs::{Motd, RakEventListenerFn, RakNetEvent, RakNetServer, RakResult, SERVER_ID};
use std::collections::HashMap;
use std::io::{Cursor, Read, Write};
use std::sync::{Arc, Mutex};

macro_rules! exp {
	($e: expr) => {
		::std::sync::Arc::make_mut(&mut $e)
	};
}
pub struct Server {
    // players on the server
    // change to actual player struct in the future
    pub players: HashMap<String, u8>,
    pub logger: Logger,
    network: Option<RakNetServer>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            logger: Logger::new("Server".to_owned()),
            network: None,
        }
    }

    pub fn recieve(&mut self, address: String, buffer: Vec<u8>) {
		let mut buf = Cursor::new(&buffer);
		// get the id of the packet
		let id = buf.read_u8().unwrap();

		let packet: GamePacket = construct_packet(id, &buffer[1..]);

		match packet {
			GamePacket::Login(pk) => {
				let data = deconstruct(pk);
				dbg!(data);
			},
			_ => return
		}
	}

    pub fn get_logger(&mut self) -> Logger {
        self.logger.clone()
    }

    pub fn get_players(&mut self) -> HashMap<String, u8> {
        self.players.clone()
    }

    fn tick(&mut self) {}
}

pub fn start(server: Arc<Mutex<Server>>, address: &str) {
		let mut server_thread = Arc::clone(&server);
		let mut raknet = RakNetServer::new(address.to_string());
		let mut s = server.lock().unwrap();
        let mut logger = Arc::new(s.get_logger().clone());
		drop(s);

        let mut logger_thread = Arc::clone(&logger);


        exp!(logger).info("Starting Server");
        let threads = raknet_start!(raknet, move |event: &RakNetEvent| {
            match event.clone() {
                RakNetEvent::Disconnect(address, reason) => {
                    exp!(logger_thread).info(
                        &format!("{} disconnected due to: {}", address, reason).to_string()[..],
                    );
                    None
                }
                RakNetEvent::GamePacket(address, buf) => {
                    let mut buffer = buf.clone();
                    let mut stream = Cursor::new(&mut buffer);
                    stream.read_u8().unwrap();
                    let result = decompress(&buffer[1..]);

                    if result.is_err() {
                        println!(
                            "Something when wrong when decoding: {}",
                            result.unwrap_err()
                        );
                        return None;
                    }
                    let decompressed = &result.unwrap();
                    let mut dstream = Cursor::new(decompressed);
                    let mut frames = Vec::<Vec<u8>>::new();
                    loop {
                        if dstream.position() as usize >= decompressed.len() {
                            break;
                        }
                        let mut position: usize = dstream.position() as usize;
                        let s: &Vec<u8> = &VarSlice::compose(&decompressed[position..], &mut position)
                            .0
                            .clone();
                        dstream.set_position(position as u64);
                        frames.push(s.to_vec());
                    }
					let mut serv = server_thread.lock().expect("not cool!");
					for frame in frames {
						// func(address.clone(), frame);
						serv.recieve(address.clone(), frame);
					}
					drop(serv);
                    Some(RakResult::Motd(Motd::default()))
                }
                _ => None,
            }
        });
        exp!(logger).info("RakNet Started.");
		let mut serv = server.as_ref().lock().unwrap();
		serv.network = Some(raknet);
        drop(serv);
        exp!(logger).info("Server started!");

		// start event loop
		threads.0.join();
		threads.1.join();

        // loop {
		// 	if let Ok(mut serv) = server.try_lock() {
        //     	serv.tick();
		// 		drop(serv);
		// 	} else {
		// 		// if the tick fails, infinitely retry until we're able to do so
		// 		loop {
		// 			// this will hang if this errors
		// 			if let Ok(mut serv) = server.try_lock() {
		// 				println!("Saved tick!");
		// 				serv.tick();
		// 				drop(serv);
		// 				break;
		// 			}
		// 		}
		// 	}
        // }
		// server ticking is bad.
		
}