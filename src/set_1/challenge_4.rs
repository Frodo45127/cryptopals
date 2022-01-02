// -------------------------------------------------------------------------------//
// Cryptopals, Set 1, Challenge 4: https://cryptopals.com/sets/1/challenges/4
// Impl by Frodo45127
// -------------------------------------------------------------------------------//

use crate::utils::*;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::PathBuf;

pub fn challenge() {
	let file = BufReader::new(File::open(PathBuf::from("assets/1-4")).unwrap());

	let mut most_scored_string = (0, String::new(), vec![]);
	for text in file.split(b'\n') {
		let text = text.unwrap();
		let processed_string = hex_array_to_byte_array(&text);

		let mut decoded_strings = vec![];
		for index in 0..255 {
			let mut result = vec![];
			processed_string.iter().for_each(|x| result.push(x ^ index));
			let string = String::from_utf8(result.to_vec());
			if let Ok(string) = string { decoded_strings.push(string); }
		}

		let most_scored = score_strings_by_frequency(&decoded_strings);
		if most_scored.0 > most_scored_string.0 {
			most_scored_string = (most_scored.0, most_scored.1, text);
		}
	}

	println!("Score: '{}'", most_scored_string.0);
	println!("String: '{}'", most_scored_string.1);
	println!("Raw data: '{:?}'", most_scored_string.2);

	println!("Challenge passed!!!");
}
