use ::async_trait::async_trait;
use ::std::io::Result;
use ::tokio::net::TcpStream;

use crate::{
	config::Config,
	modern::ModernVersion,
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
		match self.state {
			ProtocolState::Login => {},
			ProtocolState::Play => {},
		}
		Ok(())
	}
}
