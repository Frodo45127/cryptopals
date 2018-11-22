use utils::{get_byte_from_hex, decrypt_fixed_xor};

const STRING_1: &[u8; 36] = b"1c0111001f010100061a024b53535009181c";
const STRING_2: &[u8; 36] = b"686974207468652062756c6c277320657965";
const EXPECTED_RESULT: &[u8; 36] = b"746865206b696420646f6e277420706c6179";

pub fn challenge() {
	let intended_result = EXPECTED_RESULT.iter().map(|x| *get_byte_from_hex(x) & 15).collect::<Vec<u8>>();
	let result = decrypt_fixed_xor(STRING_1, STRING_2);

    assert_eq!(result, intended_result.to_vec());
}

