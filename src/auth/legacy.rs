use ::std::error::Error;

use crate::config::Config;

pub struct Legacy;

impl Legacy {
	pub async fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
		todo!();
	}
}
