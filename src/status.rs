use ::serde_json::json;
use ::std::io::Result;
use ::tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{config::Config, util};

pub async fn classic(conn: &mut TcpStream, config: &Config) -> Result<()> {
	use crate::classic::types::ClassicEncodable;
	todo!()
}

pub async fn modern(conn: &mut TcpStream, config: &Config, version: i32) -> Result<()> {
	use crate::modern::types::{ModernEncodable, VarInt};
	let version = if crate::modern::supported(version) {
		version
	} else {
		0
	};
	'status: loop {
		let _length = VarInt::async_read(conn).await?;
		let id = VarInt::async_read(conn).await?;
		match id.raw {
			0 => {
				let id = VarInt::from(0);
				let status = json!({
					"version": {
						"name": "Basalt",
						"protocol": version
					},
					"players": {
						"max": config.minecraft.max_players,
						"online": 0,
						"sample": []
					},
					"description": {
						"text": config.minecraft.motd
					}
				})
				.to_string();
				let mut buffer = Vec::new();
				id.write(&mut buffer)?;
				status.write(&mut buffer)?;
				conn.write(&util::prepend_length(buffer)).await?;
			}
			1 => {
				let payload = i64::async_read(conn).await?;
				let mut buffer = Vec::new();
				id.async_write(&mut buffer).await?;
				payload.async_write(&mut buffer).await?;
				conn.write_all(&util::prepend_length(buffer)).await?;
				break 'status;
			}
			_ => {}
		}
	}
	Ok(())
}
