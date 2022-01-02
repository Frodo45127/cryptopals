// -------------------------------------------------------------------------------//
// Cryptopals, Set 1, Challenge 5: https://cryptopals.com/sets/1/challenges/5
// Impl by Frodo45127
// -------------------------------------------------------------------------------//

use crate::utils::*;

const KEY: &[u8; 3] = b"ICE";
const STRING: &str = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
const INTENDED_RESULT: &str = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

pub fn challenge() {

	let intended_result = hex_array_to_byte_array(INTENDED_RESULT.as_bytes());
	let string = STRING.as_bytes();

	let mut encrypted_string = vec![];
	for (position, character) in string.iter().enumerate() {
		encrypted_string.push(character ^ (KEY[position % KEY.len()]));
	}

	assert_eq!(encrypted_string, intended_result);
}
