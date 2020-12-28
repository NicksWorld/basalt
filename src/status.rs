use ::std::io::Result;

use crate::connection::JavaConnection;
use crate::types::java::VarInt;

pub async fn modern(conn: &mut JavaConnection) -> Result<()> {
	let motd = String::from("{\"version\":{\"name\":\"1.16.4\",\"protocol\":754},\"players\":{\"max\":20,\"online\":0,\"sample\":[]},\"description\":{\"text\":\"Hello from Basalt!\"}}");
	conn.write(VarInt::from(motd.len() as i32 + 1)).await?;
	conn.write(0u8).await?;
	conn.write(motd).await?;
	conn.flush().await?;
	Ok(())
}
