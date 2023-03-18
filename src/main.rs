mod keys;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;

fn ask_filename(options: &mut OpenOptions, prompt: &str) -> File {
	println!("{}", prompt);
	let mut filename = String::new();
	std::io::stdin()
		.read_line(&mut filename)
		.expect("Failed to read string");
	options
		.open(Path::new(filename.trim()))
		.expect("Failed to open file")
}

fn main() {
	print!(r#"Welcome to encryption program!
What doing?
1. Generate key
2. Decrypt using key
3. Encrypt using key

Enter your choice: 
"#);
	let mut choice = String::new();
	std::io::stdin()
		.read_line(&mut choice)
		.expect("Failed to read string");
	println!("...");
	match choice
		.chars()
		.nth(0)
		.unwrap()
	{
		'1' => {
			let mut file = ask_filename(
				OpenOptions::new()
					.write(true)
					.create(true),
				"Enter file where store key: "
			);
			println!("Generating key...");
			println!("Sucessfully wrote {} bytes", file
				.write(keys::generate_key().as_slice())
				.expect("Failed to write file"));
		},
		'2' => {
			let mut file_with_key = ask_filename(
				OpenOptions::new()
					.read(true),
				"Enter file with key: "
			);
			let input_file = ask_filename(
				OpenOptions::new()
					.read(true),
				"Enter file to decrypt"
			);
			let mut output_file = ask_filename(
				OpenOptions::new()
					.write(true)
					.create(true),
				"Enter file where will stored decrypted data: "
			);
			let mut key = vec![0u8; 256];
			let readed = file_with_key.read(&mut key);
			if let Ok(count) = readed {
				if count != 256 {
					panic!("{} is not 256", count);
				}
			}
			if let Err(error) = readed {
				panic!("{:?}", error);
			}
			for byte in input_file.bytes() {
				let byte = byte.unwrap();
				output_file.write(&[
					key[byte as usize]
				]).unwrap();
			}
		},
		'3' => {
			let mut file_with_key = ask_filename(
				OpenOptions::new()
					.read(true),
				"Enter file with key: "
			);
			let input_file = ask_filename(
				OpenOptions::new()
					.read(true),
				"Enter file to encrypt: "
			);
			let mut output_file = ask_filename(
				OpenOptions::new()
					.write(true)
					.create(true),
				"Enter file where will stored encrypted data: "
			);
			let mut key = vec![0u8; 256];
			let readed = file_with_key.read(&mut key);
			if let Ok(count) = readed {
				if count != 256 {
					panic!("{} is not 256", count);
				}
			}
			if let Err(error) = readed {
				panic!("{:?}", error);
			}
			for byte in input_file.bytes() {
				let byte = byte.unwrap();
				output_file.write(&[
					key
						.iter()
						.position(|&e| e == byte)
						.unwrap() as u8
				]).unwrap();
			}
			
		},
		_ => {
			eprintln!("Invalid choice");
		},
	}
}
