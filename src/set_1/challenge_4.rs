use utils::get_byte_from_hex;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::PathBuf;

pub fn challenge() {
	let file = BufReader::new(File::open(PathBuf::from("1-4")).unwrap());

	let mut most_scored_string = (0, String::new(), vec![]);
	for text in file.split(b'\n') {
		let text = text.unwrap();
		let mut processed_string = text.clone();

		let mut y = 0;
		while y < text.len() {
			processed_string[y] = *get_byte_from_hex(&text[y]) & 15;
			processed_string[y + 1] = *get_byte_from_hex(&text[y + 1]) & 15;
			processed_string[y] <<= 4;
			processed_string[y] |= processed_string[y + 1];
			y += 2;
		}

		let processed_string = processed_string.iter().enumerate().filter(|x| x.0 % 2 == 0).map(|x| *x.1).collect::<Vec<u8>>();

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

		if most_scored.0 > most_scored_string.0 { most_scored_string = (most_scored.0, most_scored.1, text); }
	}
	println!("{:?}", most_scored_string);
}
