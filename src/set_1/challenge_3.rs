use utils::hex_string_to_byte_array;

const ENCODED_STRING: &[u8; 68] = b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

pub fn challenge() {

	let processed_string = hex_string_to_byte_array(ENCODED_STRING);

	let mut decoded_strings = vec![];
	for index in 0..255 {
		let mut result = vec![];
		processed_string.iter().for_each(|x| result.push(x ^ index));
		let string = String::from_utf8(result.to_vec());
		if let Ok(string) = string { decoded_strings.push(string); }
	}

	let mut most_scored = (0, String::new());
	let mut score = 0;
	for string in decoded_strings {
		score += string.matches("e").count() * 12;
		score += string.matches("t").count() * 11;
		score += string.matches("a").count() * 10;
		score += string.matches("o").count() * 9;
		score += string.matches("i").count() * 8;
		score += string.matches("n").count() * 7;
		score += string.matches("s").count() * 6;
		score += string.matches("h").count() * 5;
		score += string.matches("r").count() * 4;
		score += string.matches("d").count() * 3;
		score += string.matches("l").count() * 2;
		score += string.matches("u").count() * 1;
		if score > most_scored.0 { most_scored = (score, string); }
		score = 0;
	}
	println!("{:?}", most_scored);
}
