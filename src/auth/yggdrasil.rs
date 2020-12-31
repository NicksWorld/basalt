use ::openssl::{pkey::Private, rsa::Rsa};
use ::std::error::Error;

use crate::config::Config;

pub struct Yggdrasil {
	keypair: Rsa<Private>,
	url: String,
}

impl Yggdrasil {
	pub async fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
		let keypair = Rsa::generate(1024)?;
		Ok(Yggdrasil {
			keypair,
			url: config.authentication.yggdrasil.url.clone(),
		})
	}
}
