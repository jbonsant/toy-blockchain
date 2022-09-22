use bytes::{Bytes, BytesMut, BufMut};

pub fn args_to_bytes(args: std::env::Args) -> Vec<u8> {

	let mut bytes = BytesMut::new();
	
	for arg in args {
		bytes.put(format!("{}{}", arg, '\0').as_bytes());
	}

	bytes.to_vec()
}

pub fn bytes_to_args(bytes: Bytes) -> Vec<String> {

	let mut args: Vec<String> = Vec::new();

	for item in bytes.split(|c| c == &(0)) {
		if item.len() > 0 {
			args.push(String::from_utf8_lossy(item).to_string());
		}
	}

	args
}