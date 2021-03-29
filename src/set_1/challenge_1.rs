// -------------------------------------------------------------------------------//
// Cryptopals, Set 1, Challenge 1: https://cryptopals.com/sets/1/challenges/1
// Impl by Frodo45127
// -------------------------------------------------------------------------------//

use crate::utils::{encrypt_base64, hex_string_to_byte_array};

pub fn challenge() {
	let hex_source = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	let base64_intended_result = b"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
	let hex_source_as_bytes = hex_string_to_byte_array(hex_source);
	let base64_result = encrypt_base64(&hex_source_as_bytes);

    assert_eq!(base64_result, base64_intended_result.to_vec());
}
