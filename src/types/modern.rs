use ::async_trait::async_trait;
use ::std::{
	io::{Error, ErrorKind, Result},
	mem,
};
use ::tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::types::BasaltError;

#[async_trait]
pub trait ModernEncodable {
	async fn read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self>
	where
		Self: Sized;
	async fn size(&self) -> usize;
	async fn write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()>;
}

#[async_trait]
impl ModernEncodable for bool {
	async fn read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		Ok(stream.read_u8().await? != 0)
	}

	async fn size(&self) -> usize {
		mem::size_of::<Self>()
	}

	async fn write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		stream.write_u8(*self as _).await?;
		Ok(())
	}
}

#[async_trait]
impl ModernEncodable for f32 {
	async fn read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		let mut buffer = [0; 4];
		stream.read(&mut buffer).await?;
		Ok(f32::from_be_bytes(buffer))
	}

	async fn size(&self) -> usize {
		mem::size_of::<Self>()
	}

	async fn write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		let buffer = self.to_be_bytes();
		stream.write(&buffer).await?;
		Ok(())
	}
}

#[async_trait]
impl ModernEncodable for f64 {
	async fn read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		let mut buffer = [0; 8];
		stream.read(&mut buffer).await?;
		Ok(f64::from_be_bytes(buffer))
	}

	async fn size(&self) -> usize {
		mem::size_of::<Self>()
	}

	async fn write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		let buffer = self.to_be_bytes();
		stream.write(&buffer).await?;
		Ok(())
	}
}

#[async_trait]
impl ModernEncodable for i8 {
	async fn read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		Ok(stream.read_i8().await?)
	}

	async fn size(&self) -> usize {
		mem::size_of::<Self>()
	}

	async fn write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		stream.write_i8(*self).await?;
		Ok(())
	}
}

#[async_trait]
impl ModernEncodable for i16 {
	async fn read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		Ok(stream.read_i16().await?)
	}

	async fn size(&self) -> usize {
		mem::size_of::<Self>()
	}

	async fn write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		stream.write_i16(*self).await?;
		Ok(())
	}
}

#[async_trait]
impl ModernEncodable for i32 {
	async fn read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		Ok(stream.read_i32().await?)
	}

	async fn size(&self) -> usize {
		mem::size_of::<Self>()
	}

	async fn write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		stream.write_i32(*self).await?;
		Ok(())
	}
}

#[async_trait]
impl ModernEncodable for i64 {
	async fn read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		Ok(stream.read_i64().await?)
	}

	async fn size(&self) -> usize {
		mem::size_of::<Self>()
	}

	async fn write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		stream.write_i64(*self).await?;
		Ok(())
	}
}

#[async_trait]
impl ModernEncodable for String {
	async fn read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		let length = VarInt::read(stream).await?.raw;
		let mut count = 0;
		let mut buffer = Vec::new();
		while length != count {
			let temp = stream.read_u8().await?;
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

	async fn size(&self) -> usize {
		let mut total = 0;
		let length = VarInt::from(self.len() as i32);
		let body = self.len();
		total += length.size().await;
		total += body;
		total
	}

	async fn write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		let length = VarInt::from(self.len() as i32);
		let raw = self.as_bytes();
		length.write(stream).await?;
		stream.write(raw).await?;
		Ok(())
	}
}

#[async_trait]
impl ModernEncodable for u8 {
	async fn read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		Ok(stream.read_u8().await?)
	}

	async fn size(&self) -> usize {
		mem::size_of::<Self>()
	}

	async fn write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		stream.write_u8(*self).await?;
		Ok(())
	}
}

#[async_trait]
impl ModernEncodable for u16 {
	async fn read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		Ok(stream.read_u16().await?)
	}

	async fn size(&self) -> usize {
		mem::size_of::<Self>()
	}

	async fn write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		stream.write_u16(*self).await?;
		Ok(())
	}
}

#[async_trait]
impl ModernEncodable for u128 {
	async fn read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		Ok(stream.read_u128().await?)
	}

	async fn size(&self) -> usize {
		mem::size_of::<Self>()
	}

	async fn write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		stream.write_u128(*self).await?;
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
	async fn read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		let mut count = 0;
		let mut result = 0;
		let mut read;
		while {
			read = stream.read_u8().await? as u32;
			let value = read & 0x7F;
			result |= value << (7 * count);
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

	async fn size(&self) -> usize {
		let mut temp = self.raw;
		let mut count = 0;
		while {
			temp >>= 7;
			count += 1;
			temp != 0
		} {}
		count
	}

	async fn write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		let mut value = self.raw;
		while {
			let mut temp = (value & 0x7F) as u8;
			value >>= 7;
			if value != 0 {
				temp |= 0x80;
			}
			stream.write_u8(temp).await?;
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
	async fn read<R: AsyncReadExt + Send + Unpin>(stream: &mut R) -> Result<Self> {
		let mut count = 0;
		let mut result = 0;
		let mut read;
		while {
			read = stream.read_u8().await? as u64;
			let value = read & 0x7F;
			result |= value << (7 * count);
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

	async fn size(&self) -> usize {
		let mut temp = self.raw;
		let mut count = 0;
		while {
			temp >>= 7;
			count += 1;
			temp != 0
		} {}
		count
	}

	async fn write<W: AsyncWriteExt + Send + Unpin>(&self, stream: &mut W) -> Result<()> {
		let mut value = self.raw;
		while {
			let mut temp = (value & 0x7F) as u8;
			value >>= 7;
			if value != 0 {
				temp |= 0x80;
			}
			stream.write_u8(temp).await?;
			value != 0
		} {}
		Ok(())
	}
}
