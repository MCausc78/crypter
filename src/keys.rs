use rand::Rng;

pub fn generate_key() -> Vec<u8> {
	let mut rng = rand::thread_rng();
	let mut result = Vec::<u8>::new();
	for _ in 0u8..=255u8 {
		let mut value: u8;
		loop {
			value = rng.gen_range(0u8..=255u8);
			if !result.contains(&value) {
				break;
			}
		}
		result.push(value);
	}
	result
}
