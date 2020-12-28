use ::log::error;
use ::std::{
	error::Error,
	net::{IpAddr, SocketAddr},
};
use ::tokio::net::TcpListener;

use crate::connection::Connection;
use crate::status;
use crate::types::java::VarInt;

pub struct Server {
	java: TcpListener,
}

impl Server {
	pub async fn listen(&mut self) -> Result<(), Box<dyn Error>> {
		loop {
			match self.java.accept().await {
				Ok((sock, addr)) => {
					let connection = Connection::java(sock).await;
					let Connection::Java(mut java) = connection;
					let length: i32 = java.read::<VarInt>().await?.into();
					if length == 0xFE {
						// TODO: Handle legacy handshakes
					} else {
						let id: i32 = java.read::<VarInt>().await?.into();
						let version: i32 = java.read::<VarInt>().await?.into();
						let address = java.read::<String>().await?;
						let port = java.read::<u16>().await?;
						let next: i32 = java.read::<VarInt>().await?.into();
						if id == 0 {
							if next == 1 {
								status::modern(&mut java).await?;
							} else if next == 2 {
								// TODO: Join the game
							} else {
								// ...
							}
						} else {
							// ...
						}
					}
				}
				Err(e) => error!("Connection dropped: {:?}", e),
			}
		}
	}

	pub async fn new(addr: IpAddr) -> Result<Self, Box<dyn Error>> {
		let jaddr = SocketAddr::new(addr, 25565);
		let java = TcpListener::bind(jaddr).await?;
		Ok(Self { java })
	}
}
