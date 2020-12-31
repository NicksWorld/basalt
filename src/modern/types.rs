use ::async_trait::async_trait;
use ::std::{
	io::{Error, ErrorKind, Read, Result, Write},
	mem,
};
use ::tokio::io::{AsyncReadExt, AsyncWriteExt};
use ::uuid::Uuid;

use crate::types::BasaltError;

#[async_trait]
pub trait ModernEncodable {
	async fn async_read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> where Self: Sized;
	async fn async_write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()>;
	fn read<R: Read>(stream: &mut R) -> Result<Self> where Self: Sized;
	fn write<W: Write>(&self, stream: &mut W) -> Result<()>;
}

#[async_trait]
impl ModernEncodable for bool {
	async fn async_read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		Ok(stream.read_u8().await? != 0)
	}

	async fn async_write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		stream.write_u8(*self as _).await?;
		Ok(())
	}

	fn read<R: Read>(stream: &mut R) -> Result<Self> {
		let mut buffer = [0u8; mem::size_of::<Self>()];
		stream.read_exact(&mut buffer)?;
		Ok(buffer[0] != 0)
	}

	fn write<W: Write>(&self, stream: &mut W) -> Result<()> {
		let buffer = [*self as u8; 1];
		stream.write_all(&buffer)?;
		Ok(())
	}
}

#[async_trait]
impl ModernEncodable for f32 {
	async fn async_read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		let mut buffer = [0u8; mem::size_of::<Self>()];
		stream.read_exact(&mut buffer).await?;
		Ok(Self::from_be_bytes(buffer))
	}

	async fn async_write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		let buffer = self.to_be_bytes();
		stream.write_all(&buffer).await?;
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
impl ModernEncodable for f64 {
	async fn async_read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		let mut buffer = [0u8; mem::size_of::<Self>()];
		stream.read_exact(&mut buffer).await?;
		Ok(Self::from_be_bytes(buffer))
	}

	async fn async_write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		let buffer = self.to_be_bytes();
		stream.write_all(&buffer).await?;
		Ok(())
	}

	fn read<R: Read>(stream: &mut R) -> Result<Self> {
		let mut buffer = [0u8; mem::size_of::<Self>()];
		stream.read(&mut buffer)?;
		Ok(Self::from_be_bytes(buffer))
	}

	fn write<W: Write>(&self, stream: &mut W) -> Result<()> {
		let buffer = self.to_be_bytes();
		stream.write(&buffer)?;
		Ok(())
	}
}

#[async_trait]
impl ModernEncodable for i8 {
	async fn async_read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		Ok(stream.read_i8().await?)
	}

	async fn async_write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		stream.write_i8(*self).await?;
		Ok(())
	}

	fn read<R: Read>(stream: &mut R) -> Result<Self> {
		let mut buffer = [0u8; 1];
		stream.read(&mut buffer)?;
		Ok(Self::from_be_bytes(buffer))
	}

	fn write<W: Write>(&self, stream: &mut W) -> Result<()> {
		let buffer = self.to_be_bytes();
		stream.write_all(&buffer)?;
		Ok(())
	}
}

#[async_trait]
impl ModernEncodable for i16 {
	async fn async_read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		Ok(stream.read_i16().await?)
	}

	async fn async_write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		stream.write_i16(*self).await?;
		Ok(())
	}

	fn read<R: Read>(stream: &mut R) -> Result<Self> {
		let mut buffer = [0u8; mem::size_of::<Self>()];
		stream.read(&mut buffer)?;
		Ok(Self::from_be_bytes(buffer))
	}

	fn write<W: Write>(&self, stream: &mut W) -> Result<()> {
		let buffer = self.to_be_bytes();
		stream.write(&buffer)?;
		Ok(())
	}
}

#[async_trait]
impl ModernEncodable for i32 {
	async fn async_read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		Ok(stream.read_i32().await?)
	}

	async fn async_write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		stream.write_i32(*self).await?;
		Ok(())
	}

	fn read<R: Read>(stream: &mut R) -> Result<Self> {
		let mut buffer = [0u8; mem::size_of::<Self>()];
		stream.read(&mut buffer)?;
		Ok(Self::from_be_bytes(buffer))
	}

	fn write<W: Write>(&self, stream: &mut W) -> Result<()> {
		let buffer = self.to_be_bytes();
		stream.write(&buffer)?;
		Ok(())
	}
}

#[async_trait]
impl ModernEncodable for i64 {
	async fn async_read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		Ok(stream.read_i64().await?)
	}

	async fn async_write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		stream.write_i64(*self).await?;
		Ok(())
	}

	fn read<R: Read>(stream: &mut R) -> Result<Self> {
		let mut buffer = [0u8; mem::size_of::<Self>()];
		stream.read(&mut buffer)?;
		Ok(Self::from_be_bytes(buffer))
	}

	fn write<W: Write>(&self, stream: &mut W) -> Result<()> {
		let buffer = self.to_be_bytes();
		stream.write(&buffer)?;
		Ok(())
	}
}

#[async_trait]
impl ModernEncodable for String {
	async fn async_read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		let length = VarInt::async_read(stream).await?.raw;
		let mut count = 0;
		let mut buffer = Vec::new();
		while length != count {
			let temp = u8::async_read(stream).await?;
			if (temp & 0x80) == 0 {
				count += 1;
			}
			buffer.push(temp);
		}
		match String::from_utf8(buffer) {
			Ok(v) => Ok(v),
			Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
		}
	}

	async fn async_write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		let length = VarInt::from(self.len() as i32);
		let raw = self.as_bytes();
		length.async_write(stream).await?;
		stream.write_all(raw).await?;
		Ok(())
	}

	fn read<R: Read>(stream: &mut R) -> Result<Self> {
		let length = VarInt::read(stream)?.raw;
		let mut count = 0;
		let mut buffer = Vec::new();
		while length != count {
			let temp = u8::read(stream)?;
			if (temp & 0x80) == 0 {
				count += 1;
			}
			buffer.push(temp);
		}
		match String::from_utf8(buffer) {
			Ok(v) => Ok(v),
			Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
		}
	}

	fn write<W: Write>(&self, stream: &mut W) -> Result<()> {
		let length = VarInt::from(self.len() as i32);
		let raw = self.as_bytes();
		length.write(stream)?;
		stream.write(raw)?;
		Ok(())
	}
}

#[async_trait]
impl ModernEncodable for u8 {
	async fn async_read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		Ok(stream.read_u8().await?)
	}

	async fn async_write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		stream.write_u8(*self).await?;
		Ok(())
	}

	fn read<R: Read>(stream: &mut R) -> Result<Self> {
		let mut buffer = [0u8; mem::size_of::<Self>()];
		stream.read(&mut buffer)?;
		Ok(Self::from_be_bytes(buffer))
	}

	fn write<W: Write>(&self, stream: &mut W) -> Result<()> {
		let buffer = self.to_be_bytes();
		stream.write(&buffer)?;
		Ok(())
	}
}

#[async_trait]
impl ModernEncodable for u16 {
	async fn async_read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		Ok(stream.read_u16().await?)
	}

	async fn async_write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		stream.write_u16(*self).await?;
		Ok(())
	}

	fn read<R: Read>(stream: &mut R) -> Result<Self> {
		let mut buffer = [0u8; mem::size_of::<Self>()];
		stream.read(&mut buffer)?;
		Ok(Self::from_be_bytes(buffer))
	}

	fn write<W: Write>(&self, stream: &mut W) -> Result<()> {
		let buffer = self.to_be_bytes();
		stream.write(&buffer)?;
		Ok(())
	}
}

#[async_trait]
impl ModernEncodable for Uuid {
	async fn async_read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		Ok(Uuid::from_u128(stream.read_u128().await?))
	}

	async fn async_write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		stream.write_u128(self.as_u128()).await?;
		Ok(())
	}

	fn read<R: Read>(stream: &mut R) -> Result<Self> {
		let mut buffer = [0u8; mem::size_of::<Self>()];
		stream.read(&mut buffer)?;
		Ok(Self::from_u128(u128::from_be_bytes(buffer)))
	}

	fn write<W: Write>(&self, stream: &mut W) -> Result<()> {
		let buffer = self.as_u128().to_be_bytes();
		stream.write(&buffer)?;
		Ok(())
	}
}

pub struct VarInt {
	pub raw: i32,
}

impl From<i32> for VarInt {
	fn from(raw: i32) -> Self {
		Self { raw }
	}
}

impl Into<i32> for VarInt {
	fn into(self) -> i32 {
		self.raw
	}
}

#[async_trait]
impl ModernEncodable for VarInt {
	async fn async_read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		let mut count = 0;
		let mut result = 0u32;
		let mut read;
		while {
			read = u8::async_read(stream).await?;
			let value = read & 0x7F;
			result |= (value as u32) << (7 * count);
			count += 1;
			if count > 5 {
				return Err(Error::new(
					ErrorKind::InvalidData,
					BasaltError::new(String::from("VarInt is too big!")),
				));
			}
			(read & 0x80) != 0
		} {}
		Ok(Self {
			raw: unsafe { mem::transmute(result) },
		})
	}

	async fn async_write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		let mut value = self.raw;
		while {
			let mut temp = (value & 0x7F) as u8;
			value >>= 7;
			if value != 0 {
				temp |= 0x80;
			}
			temp.async_write(stream).await?;
			value != 0
		} {}
		Ok(())
	}

	fn read<R: Read>(stream: &mut R) -> Result<Self> {
		let mut count = 0;
		let mut result = 0u32;
		let mut read;
		while {
			read = u8::read(stream)?;
			let value = read & 0x7F;
			result |= (value as u32) << (7 * count);
			count += 1;
			if count > 5 {
				return Err(Error::new(
					ErrorKind::InvalidData,
					BasaltError::new(String::from("VarInt is too big!")),
				));
			}
			(read & 0x80) != 0
		} {}
		Ok(Self {
			raw: unsafe { mem::transmute(result) },
		})
	}

	fn write<W: Write>(&self, stream: &mut W) -> Result<()> {
		let mut value = self.raw;
		while {
			let mut temp = (value & 0x7F) as u8;
			value >>= 7;
			if value != 0 {
				temp |= 0x80;
			}
			temp.write(stream)?;
			value != 0
		} {}
		Ok(())
	}
}

pub struct VarLong {
	pub raw: i64,
}

impl From<i64> for VarLong {
	fn from(raw: i64) -> Self {
		Self { raw }
	}
}

impl Into<i64> for VarLong {
	fn into(self) -> i64 {
		self.raw
	}
}

#[async_trait]
impl ModernEncodable for VarLong {
	async fn async_read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		let mut count = 0;
		let mut result = 0u64;
		let mut read;
		while {
			read = u8::async_read(stream).await?;
			let value = read & 0x7F;
			result |= (value as u64) << (7 * count);
			count += 1;
			if count > 10 {
				return Err(Error::new(
					ErrorKind::InvalidData,
					BasaltError::new(String::from("VarInt is too big!")),
				));
			}
			(read & 0x80) != 0
		} {}
		Ok(Self {
			raw: unsafe { mem::transmute(result) },
		})
	}

	async fn async_write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		let mut value = self.raw;
		while {
			let mut temp = (value & 0x7F) as u8;
			value >>= 7;
			if value != 0 {
				temp |= 0x80;
			}
			temp.async_write(stream).await?;
			value != 0
		} {}
		Ok(())
	}

	fn read<R: Read>(stream: &mut R) -> Result<Self> {
		let mut count = 0;
		let mut result = 0u64;
		let mut read;
		while {
			read = u8::read(stream)?;
			let value = read & 0x7F;
			result |= (value as u64) << (7 * count);
			count += 1;
			if count > 10 {
				return Err(Error::new(
					ErrorKind::InvalidData,
					BasaltError::new(String::from("VarInt is too big!")),
				));
			}
			(read & 0x80) != 0
		} {}
		Ok(Self {
			raw: unsafe { mem::transmute(result) },
		})
	}

	fn write<W: Write>(&self, stream: &mut W) -> Result<()> {
		let mut value = self.raw;
		while {
			let mut temp = (value & 0x7F) as u8;
			value >>= 7;
			if value != 0 {
				temp |= 0x80;
			}
			temp.write(stream)?;
			value != 0
		} {}
		Ok(())
	}
}
