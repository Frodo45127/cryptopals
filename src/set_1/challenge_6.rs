use utils::decrypt_base64;
use std::io::{BufReader, Read};
use std::fs::File;
use std::path::PathBuf;

pub fn challenge() {

	// Get the file in a Vec we can actually manipulate.
	let mut file = BufReader::new(File::open(PathBuf::from("1-6")).unwrap());
	let mut data_to_decrypt = vec![];
	file.read_to_end(&mut data_to_decrypt).unwrap();
	
	// First, we get rid of the base64 encoding.
	let data_decrypted_base64 = decrypt_base64(&data_to_decrypt);

	// Then, ensure our function to calculate the hamming distance is correct.
	let string_1 = b"this is a test";
	let string_2 = b"wokka wokka!!!";

	let zip = string_1.iter().zip(string_2.iter());
	let hamming_distance = zip.fold(0, |acc, (x, y)| acc + (x ^ y).count_ones());
	assert_eq!(hamming_distance, 37);

	// Now, find the XOR KeySize.
	let mut key_size = 99999;
	let mut normal_big = 99999;

	for size in 2..40 {

		let mut y = 0;
		let mut normal = 0;
		while data_decrypted_base64.get(size * (y + 2)).is_some() {
			let range_1 = &data_decrypted_base64[(size * y)..(size * (y + 1))];
			let range_2 = &data_decrypted_base64[(size * (y + 1))..(size * (y + 2))];

			let zip = range_1.iter().zip(range_2.iter());
			let hamming_distance = zip.fold(0, |acc, (x, y)| acc + (x ^ y).count_ones());
			normal += hamming_distance / size as u32;
			y += 1;
		}

		normal /= (y / 2) as u32;
		if normal < normal_big { key_size = size as u32; normal_big = normal; }
	}

	let mut transposed_data = vec![];
	for index in 0..key_size {
		transposed_data.push(
			data_decrypted_base64.iter().enumerate()
				.filter(|(y, _)| 
					(
						*y as isize - index as isize) 
						% key_size as isize == 0
					)
				.map(|x| *x.1)
				.collect::<Vec<u8>>());
	}

	// Now, decode every character of the key, one by one.
	let mut key = vec![];
	for data in &transposed_data {

		let mut decoded_strings = vec![];
		for index in 0..255 {
			let mut result = vec![];
			data.iter().for_each(|x| result.push(x ^ index));
			let string = String::from_utf8(result.to_vec());
			if let Ok(string) = string { decoded_strings.push((index, string)); }
		}

		let mut most_scored = (0, 0, String::new());
		let mut score = 0;
		for (index, string) in decoded_strings {
			score += string.matches("e").count() * 12;
			score += string.matches("t").count() * 12;
			score += string.matches("a").count() * 12;
			score += string.matches("o").count() * 12;
			score += string.matches("i").count() * 12;
			score += string.matches("n").count() * 12;
			score += string.matches("s").count() * 12;
			score += string.matches("h").count() * 12;
			score += string.matches("r").count() * 12;
			score += string.matches("d").count() * 12;
			score += string.matches("l").count() * 12;
			score += string.matches("u").count() * 12;
			if score > most_scored.0 { most_scored = (score, index, string); }
			score = 0;
		}

		key.push(most_scored.1);
	}

	println!("Key: {:?}", String::from_utf8_lossy(&key));

	let mut decrypted_string = vec![];
	for (position, character) in data_decrypted_base64.iter().enumerate() {
		decrypted_string.push(character ^ (key[position % key.len()]));
	}

	println!("Decrypted Text:\n{}", String::from_utf8_lossy(&decrypted_string));

}
