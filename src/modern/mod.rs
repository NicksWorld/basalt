use ::async_trait::async_trait;
use ::tokio::net::TcpStream;

use crate::{
	config::Config,
	types::ProtocolHandler,
};

mod dummy;
pub mod types;
mod v754;

use dummy::DummyHandler;

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

const SUPPORTED_VERSIONS: &[i32] = &[754];

pub fn supported(version: i32) -> bool {
	SUPPORTED_VERSIONS.contains(&version)
}
