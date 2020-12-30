use ::async_trait::async_trait;

use crate::{
	config::Config,
	connection::ModernConnection,
	types::ProtocolHandler,
};

pub mod types;
mod v754;

pub use v754::V754;

#[async_trait]
pub trait ModernVersion {
	async fn new(conn: ModernConnection, config: &Config) -> Box<Self>;
}

pub async fn handler(conn: ModernConnection, config: &Config, version: i32) -> Box<dyn ProtocolHandler> {
	match version {
		754 => V754::new(conn, config).await,
		_ => DummyHandler::new(conn, config).await,
	}
}

struct DummyHandler {
	conn: ModernConnection
}

#[async_trait]
impl ModernVersion for DummyHandler {
	async fn new(conn: ModernConnection, _config: &Config) -> Box<Self> {
		Box::new(Self {
			conn
		})
	}
}

impl ProtocolHandler for DummyHandler {}
