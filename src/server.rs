use ::log::error;
use ::std::{error::Error, net::SocketAddr};
use ::tokio::{self, net::TcpListener};

use crate::{
	auth::Authentication,
	config::Config,
	connection::Connection,
	modern::{
		self,
		types::{ModernEncodable, VarInt},
	},
	status,
};

pub struct Server {
	auth: Authentication,
	java: TcpListener,
}

impl Server {
	pub async fn listen(&mut self, config: &Config) -> Result<(), Box<dyn Error>> {
		loop {
			match self.java.accept().await {
				Ok((sock, _addr)) => {
					let config = config.clone();
					tokio::spawn(async move {
						let connection = Connection::java(sock).await.unwrap();
						match connection {
							Connection::Classic(mut conn) => {
								let id = u8::async_read(&mut conn).await.unwrap();
								if id == 0xFE {
									status::classic(&mut conn, &config).await.unwrap();
								} else if id == 0x00 {
									// TODO: Join the game
								}
							}
							Connection::Modern(mut conn) => {
								let _length: i32 =
									VarInt::async_read(&mut conn).await.unwrap().into();
								let id: i32 = VarInt::async_read(&mut conn).await.unwrap().into();
								let version: i32 =
									VarInt::async_read(&mut conn).await.unwrap().into();
								let _address = String::async_read(&mut conn).await.unwrap();
								let _port = u16::async_read(&mut conn).await.unwrap();
								let next: i32 = VarInt::async_read(&mut conn).await.unwrap().into();
								if id == 0 {
									if next == 1 {
										status::modern(&mut conn, &config, version).await.unwrap();
									} else if next == 2 {
										let handler = modern::handler(conn, &config, version).await;
										// TODO: Pass the handler to the player object
										todo!();
									} else {
										// ...
									}
								} else {
									// ...
								}
							}
						}
					});
				}
				Err(e) => error!("Connection dropped: {:?}", e),
			}
		}
	}

	pub async fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
		let auth = Authentication::new(config).await?;
		let jaddr = SocketAddr::new(config.network.bind.parse().unwrap(), config.network.port);
		let java = TcpListener::bind(jaddr).await?;
		Ok(Self { auth, java })
	}
}
