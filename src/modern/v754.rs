use ::async_trait::async_trait;
use ::std::io::Result;
use ::tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{
	config::Config,
	modern::{
		types::{ModernEncodable, VarInt},
		ModernVersion,
	},
	types::{ProtocolHandler, ProtocolState},
};

pub struct V754 {
	config: Config,
	conn: TcpStream,
	state: ProtocolState,
}

#[async_trait]
impl ModernVersion for V754 {
	async fn new(conn: TcpStream, config: &Config) -> Box<Self> {
		Box::new(Self {
			config: config.clone(),
			conn,
			state: ProtocolState::Login,
		})
	}
}

#[async_trait]
impl ProtocolHandler for V754 {
	async fn disconnect(&mut self, reason: String) -> Result<()> {
		let mut buffer = Vec::new();
		match self.state {
			ProtocolState::Login => {
				VarInt::from(0x00).write(&mut buffer)?;
			}
			ProtocolState::Play => {
				VarInt::from(0x19).write(&mut buffer)?;
			}
		}
		format!("{{\"text\":\"{}\"}}", reason).write(&mut buffer)?;
		self.conn.write_all(&buffer).await?;
		Ok(())
	}
}
