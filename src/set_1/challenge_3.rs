// -------------------------------------------------------------------------------//
// Cryptopals, Set 1, Challenge 3: https://cryptopals.com/sets/1/challenges/3
// Impl by Frodo45127
// -------------------------------------------------------------------------------//

use crate::utils::*;

const ENCODED_STRING: &[u8; 68] = b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

pub fn challenge() {

	let processed_string = hex_array_to_byte_array(ENCODED_STRING);

	let mut decoded_strings = vec![];
	for index in 0..255 {
		let mut result = vec![];
		processed_string.iter().for_each(|x| result.push(x ^ index));
		let string = String::from_utf8(result.to_vec());
		if let Ok(string) = string { decoded_strings.push(string); }
	}

	let most_scored = score_strings_by_frequency(&decoded_strings);

	println!("Score: '{}'", most_scored.0);
	println!("String: '{}'", most_scored.1);

	println!("Challenge passed!!!");
}
