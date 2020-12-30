use ::std::error::Error;

use crate::config::Config;

pub struct Microsoft;

impl Microsoft {
	pub async fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
		todo!();
	}
}
