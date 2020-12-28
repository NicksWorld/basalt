use ::async_trait::async_trait;
use ::std::io::Result;

use crate::config::Config;
use crate::connection::ModernConnection;

mod v754;

pub use v754::V754;

#[async_trait]
pub trait ModernVersion {
	async fn handle(conn: ModernConnection, config: &Config) -> Result<()>;
}

pub async fn handle(conn: ModernConnection, config: &Config, version: i32) -> Result<()> {
	Ok(match version {
		754 => V754::handle(conn, config).await?,
		_ => (),
	})
}
