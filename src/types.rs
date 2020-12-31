use ::async_trait::async_trait;
use ::std::{
	error::Error,
	fmt::{self, Display, Formatter},
	io,
};

#[derive(Debug)]
pub struct BasaltError {
	reason: String,
}

impl BasaltError {
	pub fn new(reason: String) -> Self {
		Self { reason }
	}
}

impl Display for BasaltError {
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		write!(f, "BasaltError: {}", self.reason)
	}
}

impl Error for BasaltError {}

#[async_trait]
pub trait ProtocolHandler {
	async fn disconnect(&mut self, reason: String) -> io::Result<()>;
	fn is_dummy(&self) -> bool {
		false
	}
}

pub enum ProtocolState {
	Login,
	Play,
}
