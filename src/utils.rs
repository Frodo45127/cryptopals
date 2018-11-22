use BASE64_TABLE;

pub fn get_byte_from_hex(byte: &u8) -> &u8 {
	match byte {
		b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7'| b'8'| b'9' => byte,
		b'a' | b'A' => &10,
		b'b' | b'B' => &11,
		b'c' | b'C' => &12,
		b'd' | b'D' => &13,
		b'e' | b'E' => &14,
		b'f' | b'F' | _ => &15,
	}
}

pub fn hex_string_to_byte_array(list: &[u8]) -> Vec<u8> {

	let mut text = list.to_vec();

	let mut y = 0;
	while y < list.len() {
		if list.get(y + 1).is_some() {
			text[y] = *get_byte_from_hex(&list[y]) & 15;
			text[y + 1] = *get_byte_from_hex(&list[y + 1]) & 15;
			text[y] <<= 4;
			text[y] |= text[y + 1];
			y += 2;
		}

		else {
			text[y] = *get_byte_from_hex(&list[y]) & 15;
			text[y] <<= 4;
			break;
		}
	}

	text.iter().enumerate().filter(|x| x.0 % 2 == 0).map(|x| *x.1).collect::<Vec<u8>>()
}

pub fn encrypt_base64(data: &[u8]) -> Vec<u8> {

	let mut encrypted_data: Vec<u8> = vec![];

	let mut index = 0;
	while index < data.len() {
		
		// Otherwise, we have an incomplete range.
		if data.get(index + 2).is_some() {
			let range = &data[index..index + 3];
			let mut bits: u32 = 0;
			for (cycle, value) in range.iter().enumerate() {
				bits |= *value as u32;
				if cycle != 2 { bits <<= 8; }
			}

			let mut x = vec![];
			let mut y = 0;
			while y < 4 {
				x.push(BASE64_TABLE[(bits & 63) as usize]);
				if y < 3 { bits >>= 6 };
				y += 1;
			}
			x.reverse();
			encrypted_data.append(&mut x);
			index += 3;
		}
		else { break; }
	}
	encrypted_data
}

pub fn decrypt_fixed_xor(string_1: &[u8], string_2: &[u8]) -> Vec<u8> {
	let mut result = vec![];
	let zip = string_1.iter().zip(string_2.iter());
	zip.for_each(|(x, y)| result.push((*get_byte_from_hex(&x) & 15) ^ (*get_byte_from_hex(&y) & 15)));
	result
}
