use ::async_trait::async_trait;
use ::log::info;
use ::std::io::{Error, ErrorKind, Result};

use crate::config::Config;
use crate::protocol::modern::{ModernConnection, ModernVersion};
use crate::types::{
	modern::{ModernEncodable, VarInt},
	BasaltError,
};

pub struct V754;

#[async_trait]
impl ModernVersion for V754 {
	async fn handle(mut conn: ModernConnection, config: &Config) -> Result<()> {
		'login: loop {
			let length: i32 = conn.read::<VarInt>().await?.into();
			let id: i32 = conn.read::<VarInt>().await?.into();
			match id {
				0 => {
					let username = conn.read::<String>().await?;
					if config.whitelist.enabled {
						if !config.whitelist.list.contains(&username) {
							info!("{} was not whitelisted!", username);
							let id = VarInt::from(0);
							let reason = String::from(
								"{\"text\":\"You are not whitelisted on this server!\"}",
							);
							let length = id.size().await + reason.size().await;
							conn.write(VarInt::from(length as i32)).await?;
							conn.write(id).await?;
							conn.write(reason).await?;
							conn.flush().await?;
							return Ok(());
						}
					}
					info!("{} is joining", username);
					let id = VarInt::from(0);
					let reason = String::from(
						"{\"text\":\"Joining the game has not been implemented yet!\"}",
					);
					let length = id.size().await + reason.size().await;
					conn.write(VarInt::from(length as i32)).await?;
					conn.write(id).await?;
					conn.write(reason).await?;
					conn.flush().await?;
					return Ok(());
				}
				_ => {
					return Err(Error::new(
						ErrorKind::InvalidData,
						BasaltError::new(String::from("Invalid packet ID")),
					))
				}
			}
		}
	}
}
