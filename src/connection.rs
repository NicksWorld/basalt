use ::std::io::Result;
use ::tokio::net::TcpStream;

pub enum Connection {
	Classic(TcpStream),
	Modern(TcpStream),
}

impl Connection {
	pub async fn java(stream: TcpStream) -> Result<Self> {
		let mut buffer = [0u8; 1];
		stream.peek(&mut buffer).await?;
		if buffer[0] == 0xFE || buffer[0] == 0x00 {
			Ok(Self::Classic(stream))
		} else {
			Ok(Self::Modern(stream))
		}
	}
}
