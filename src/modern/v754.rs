use ::async_trait::async_trait;

use crate::{
	config::Config,
	modern::{ModernConnection, ModernVersion},
	types::ProtocolHandler,
};

pub struct V754 {
	config: Config,
	conn: ModernConnection
}

#[async_trait]
impl ModernVersion for V754 {
	async fn new(conn: ModernConnection, config: &Config) -> Box<Self> {
		Box::new(Self {
			config: config.clone(),
			conn,
		})
	}
}

#[async_trait]
impl ProtocolHandler for V754 {}
