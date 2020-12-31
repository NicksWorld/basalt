use crate::modern::types::{ModernEncodable, VarInt};

pub fn prepend_length(buffer: Vec<u8>) -> Vec<u8> {
	let mut result = Vec::new();
	VarInt::from(buffer.len() as i32)
		.write(&mut result)
		.unwrap();
	result.extend_from_slice(&buffer);
	result
}
