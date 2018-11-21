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

pub fn hex_string_to_byte_array(list: &mut Vec<u8>) {

	let text = list.clone();

	let mut y = 0;
	while y < text.len() {
		if text.get(y + 1).is_some() {
			list[y] = *get_byte_from_hex(&text[y]) & 15;
			list[y + 1] = *get_byte_from_hex(&text[y + 1]) & 15;
			list[y] <<= 4;
			list[y] |= list[y + 1];
			y += 2;
		}

		else {
			list[y] = *get_byte_from_hex(&text[y]) & 15;
			list[y] <<= 4;
			break;
		}
	}

	*list = list.iter().enumerate().filter(|x| x.0 % 2 == 0).map(|x| *x.1).collect::<Vec<u8>>();
}
