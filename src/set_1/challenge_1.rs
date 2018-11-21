use utils::get_byte_from_hex;

pub fn challenge() {
    let base64table = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
	let hex_source = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	let base64_intended_result = b"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
	let mut base64_result: Vec<u8> = vec![];

	let mut index = 0;
	while index < hex_source.len() {
		
		// Otherwise, we have an incomplete range.
		if hex_source.get(index + 5).is_some() {
			let range = &hex_source[index..index + 6];
			let mut bits: u32 = 0;
			for (cycle, value) in range.iter().enumerate() {
				bits |= (*get_byte_from_hex(&value) & 15) as u32;
				if cycle != 5 { bits <<= 4; }
			}

			let mut x = vec![];
			let mut y = 0;
			while y < 4 {
				x.push(base64table[(bits & 63) as usize]);
				if y < 3 { bits >>= 6 };
				y += 1;
			}
			x.reverse();
			base64_result.append(&mut x);
			index += 6;
		}
		else { break; }
	}
    assert_eq!(base64_result, base64_intended_result.to_vec());
}
