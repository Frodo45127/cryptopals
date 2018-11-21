use utils::get_byte_from_hex;

pub fn challenge() {
	let key = b"ICE";
	let string = b"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
	let mut intended_result = b"0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".to_vec();
	let text = intended_result.to_vec();

	let mut y = 0;
	while y < text.len() {
		if text.get(y + 1).is_some() {
			intended_result[y] = *get_byte_from_hex(&text[y]) & 15;
			intended_result[y + 1] = *get_byte_from_hex(&text[y + 1]) & 15;
			intended_result[y] <<= 4;
			intended_result[y] |= intended_result[y + 1];
			y += 2;
		}

		else {
			intended_result[y] = *get_byte_from_hex(&text[y]) & 15;
			intended_result[y] <<= 4;
			break;
		}
	}

	let intended_result = intended_result.iter().enumerate().filter(|x| x.0 % 2 == 0).map(|x| *x.1).collect::<Vec<u8>>();

	let mut encrypted_string = vec![];
	for (position, character) in string.iter().enumerate() {
		encrypted_string.push(character ^ (key[position % key.len()]));
	}

	assert_eq!(encrypted_string, intended_result.to_vec());
}
