use ::std::io::Result;
use ::tokio::{
	io::{AsyncWriteExt, BufWriter},
	net::TcpStream,
};

use crate::{classic::types::ClassicEncodable, modern::types::ModernEncodable};

pub enum Connection {
	Classic(ClassicConnection),
	Modern(ModernConnection),
}

impl Connection {
	pub async fn java(stream: TcpStream) -> Result<Self> {
		let mut buffer = [0u8; 1];
		stream.peek(&mut buffer).await?;
		if buffer[0] == 0xFE || buffer[0] == 0x00 {
			Ok(Self::Classic(ClassicConnection::new(stream).await?))
		} else {
			Ok(Self::Modern(ModernConnection::new(stream).await?))
		}
	}
}

pub struct ClassicConnection {
	stream: BufWriter<TcpStream>,
}

impl ClassicConnection {
	pub async fn flush(&mut self) -> Result<()> {
		self.stream.flush().await?;
		Ok(())
	}

	pub async fn new(stream: TcpStream) -> Result<Self> {
		let stream = BufWriter::new(stream);
		Ok(Self { stream })
	}

	pub async fn read<T: ClassicEncodable>(&mut self) -> Result<T> {
		Ok(T::read(&mut self.stream).await?)
	}

	pub async fn write<T: ClassicEncodable>(&mut self, victim: T) -> Result<()> {
		victim.write(&mut self.stream).await?;
		Ok(())
	}
}

pub struct ModernConnection {
	stream: BufWriter<TcpStream>,
}

impl ModernConnection {
	pub async fn flush(&mut self) -> Result<()> {
		self.stream.flush().await?;
		Ok(())
	}

	pub async fn new(stream: TcpStream) -> Result<Self> {
		let stream = BufWriter::new(stream);
		Ok(Self { stream })
	}

	pub async fn read<T: ModernEncodable>(&mut self) -> Result<T> {
		Ok(T::read(&mut self.stream).await?)
	}

	pub async fn write<T: ModernEncodable>(&mut self, victim: T) -> Result<()> {
		victim.write(&mut self.stream).await?;
		Ok(())
	}
}
