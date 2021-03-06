use ::async_trait::async_trait;
use ::std::{
	io::{Error, ErrorKind, Read, Result, Write},
	mem,
};
use ::tokio::io::{AsyncReadExt, AsyncWriteExt};

#[async_trait]
pub trait ClassicEncodable {
	async fn async_read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self>
	where
		Self: Sized;
	async fn async_write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()>;
	fn read<R: Read>(stream: &mut R) -> Result<Self>
	where
		Self: Sized;
	fn write<W: Write>(&self, stream: &mut W) -> Result<()>;
}

#[async_trait]
impl ClassicEncodable for i8 {
	async fn async_read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		Ok(stream.read_i8().await?)
	}

	async fn async_write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		stream.write_i8(*self).await?;
		Ok(())
	}

	fn read<R: Read>(stream: &mut R) -> Result<Self> {
		let mut buffer = [0u8; mem::size_of::<Self>()];
		stream.read_exact(&mut buffer)?;
		Ok(Self::from_be_bytes(buffer))
	}

	fn write<W: Write>(&self, stream: &mut W) -> Result<()> {
		let buffer = self.to_be_bytes();
		stream.write_all(&buffer)?;
		Ok(())
	}
}

#[async_trait]
impl ClassicEncodable for i16 {
	async fn async_read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		Ok(stream.read_i16().await?)
	}

	async fn async_write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		stream.write_i16(*self).await?;
		Ok(())
	}

	fn read<R: Read>(stream: &mut R) -> Result<Self> {
		let mut buffer = [0u8; mem::size_of::<Self>()];
		stream.read_exact(&mut buffer)?;
		Ok(Self::from_be_bytes(buffer))
	}

	fn write<W: Write>(&self, stream: &mut W) -> Result<()> {
		let buffer = self.to_be_bytes();
		stream.write_all(&buffer)?;
		Ok(())
	}
}

#[async_trait]
impl ClassicEncodable for String {
	async fn async_read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		let mut buffer8 = [0u8; 64];
		stream.read(&mut buffer8).await?;
		let buffer16: [u16; 32] = unsafe { mem::transmute(buffer8) };
		match String::from_utf16(&buffer16) {
			Ok(v) => Ok(v),
			Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
		}
	}

	async fn async_write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		for c in self.as_str().encode_utf16() {
			stream.write_u16(c).await?;
		}
		Ok(())
	}

	fn read<R: Read>(stream: &mut R) -> Result<Self> {
		let mut buffer8 = [0u8; 64];
		stream.read(&mut buffer8)?;
		let buffer16: [u16; 32] = unsafe { mem::transmute(buffer8) };
		match String::from_utf16(&buffer16) {
			Ok(v) => Ok(v),
			Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
		}
	}

	fn write<W: Write>(&self, stream: &mut W) -> Result<()> {
		for c in self.as_str().encode_utf16() {
			stream.write_all(&c.to_be_bytes())?;
		}
		Ok(())
	}
}

#[async_trait]
impl ClassicEncodable for u8 {
	async fn async_read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		Ok(stream.read_u8().await?)
	}

	async fn async_write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		stream.write_u8(*self).await?;
		Ok(())
	}

	fn read<R: Read>(stream: &mut R) -> Result<Self> {
		let mut buffer = [0u8; mem::size_of::<Self>()];
		stream.read_exact(&mut buffer)?;
		Ok(Self::from_be_bytes(buffer))
	}

	fn write<W: Write>(&self, stream: &mut W) -> Result<()> {
		let buffer = self.to_be_bytes();
		stream.write_all(&buffer)?;
		Ok(())
	}
}

#[async_trait]
impl ClassicEncodable for [u8; 1024] {
	async fn async_read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		let mut buffer = [0u8; 1024];
		stream.read_exact(&mut buffer).await?;
		Ok(buffer)
	}

	async fn async_write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		stream.write_all(self).await?;
		Ok(())
	}

	fn read<R: Read>(stream: &mut R) -> Result<Self> {
		let mut buffer = [0u8; 1024];
		stream.read_exact(&mut buffer)?;
		Ok(buffer)
	}

	fn write<W: Write>(&self, stream: &mut W) -> Result<()> {
		stream.write_all(self)?;
		Ok(())
	}
}
