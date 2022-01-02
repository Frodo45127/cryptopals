// -------------------------------------------------------------------------------//
// Cryptopals, Set 1, Challenge 2: https://cryptopals.com/sets/1/challenges/2
// Impl by Frodo45127
// -------------------------------------------------------------------------------//

use crate::utils::*;

const STRING_1: &[u8; 36] = b"1c0111001f010100061a024b53535009181c";
const STRING_2: &[u8; 36] = b"686974207468652062756c6c277320657965";
const EXPECTED_RESULT: &[u8; 36] = b"746865206b696420646f6e277420706c6179";

pub fn challenge() {
	let intended_result = EXPECTED_RESULT.iter().map(|x| get_byte_from_hex(x)).collect::<Vec<u8>>();
	let data_1 = STRING_1.iter().map(|x| get_byte_from_hex(x)).collect::<Vec<u8>>();
	let data_2 = STRING_2.iter().map(|x| get_byte_from_hex(x)).collect::<Vec<u8>>();
	let result = xor_data(&data_1, &data_2);

	println!("Result value:   '{:?}'", result);
	println!("Expected value: '{:?}'", intended_result);

    assert_eq!(result, intended_result);

    println!("Challenge passed!!!");
}

