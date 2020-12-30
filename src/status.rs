use ::serde_json::json;
use ::std::io::Result;

use crate::config::Config;
use crate::connection::{ClassicConnection, ModernConnection};

pub async fn classic(conn: &mut ClassicConnection, config: &Config) -> Result<()> {
	use crate::classic::types::ClassicEncodable;
	todo!();
	Ok(())
}

pub async fn modern(conn: &mut ModernConnection, config: &Config, version: i32) -> Result<()> {
	use crate::modern::types::{ModernEncodable, VarInt};
	'status: loop {
		let length = conn.read::<VarInt>().await?;
		let id = conn.read::<VarInt>().await?;
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
				let length = id.size().await + status.size().await;
				conn.write(VarInt::from(length as i32)).await?;
				conn.write(id).await?;
				conn.write(status).await?;
				conn.flush().await?;
			}
			1 => {
				let payload = conn.read::<i64>().await?;
				conn.write(length).await?;
				conn.write(id).await?;
				conn.write(payload).await?;
				conn.flush().await?;
				break 'status;
			}
			_ => {}
		}
	}
	Ok(())
}
