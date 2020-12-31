use ::async_trait::async_trait;
use ::std::io::Result;
use ::tokio::net::TcpStream;

use crate::{
	config::Config,
	types::ProtocolHandler,
};

pub mod types;
mod v754;

pub use v754::V754;

#[async_trait]
pub trait ModernVersion {
	async fn new(conn: TcpStream, config: &Config) -> Box<Self>;
}

pub async fn handler(conn: TcpStream, config: &Config, version: i32) -> Box<dyn ProtocolHandler> {
	match version {
		754 => V754::new(conn, config).await,
		_ => DummyHandler::new(conn, config).await,
	}
}

struct DummyHandler {
	conn: TcpStream
}

#[async_trait]
impl ModernVersion for DummyHandler {
	async fn new(conn: TcpStream, _config: &Config) -> Box<Self> {
		Box::new(Self {
			conn
		})
	}
}

#[async_trait]
impl ProtocolHandler for DummyHandler {
	async fn disconnect(&mut self, reason: String) -> Result<()> {
		todo!()
	}
}
