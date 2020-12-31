use ::async_trait::async_trait;
use ::std::io::Result;
use ::tokio::net::TcpStream;

use crate::{
	config::Config,
	modern::{ModernVersion, ProtocolHandler},
};

pub struct DummyHandler {
	conn: TcpStream,
}

#[async_trait]
impl ModernVersion for DummyHandler {
	async fn new(conn: TcpStream, _config: &Config) -> Box<Self> {
		Box::new(Self { conn })
	}
}

#[async_trait]
impl ProtocolHandler for DummyHandler {
	async fn disconnect(&mut self, reason: String) -> Result<()> {
		todo!()
	}

	fn is_dummy(&self) -> bool {
		true
	}
}
