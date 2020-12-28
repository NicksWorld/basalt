use ::std::io::Result;
use ::tokio::{io::{AsyncWriteExt, BufWriter}, net::TcpStream};

use crate::types::java::JavaEncodable;

pub enum Connection {
	Java(JavaConnection),
}

impl Connection {
	pub async fn java(stream: TcpStream) -> Self {
		Self::Java(JavaConnection::new(stream).await)
	}
}

pub struct JavaConnection {
	stream: BufWriter<TcpStream>,
}

impl JavaConnection {
	pub async fn flush(&mut self) -> Result<()> {
		self.stream.flush().await?;
		Ok(())
	}

	pub async fn new(stream: TcpStream) -> Self {
		Self {
			stream: BufWriter::new(stream)
		}
	}

	pub async fn read<T: JavaEncodable>(&mut self) -> Result<T> {
		Ok(T::read(&mut self.stream).await?)
	}

	pub async fn write<T: JavaEncodable>(&mut self, victim: T) -> Result<()> {
		victim.write(&mut self.stream).await?;
		Ok(())
	}
}
