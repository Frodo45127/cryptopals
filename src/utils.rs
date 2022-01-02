// -------------------------------------------------------------------------------//
// Module containing generic functions not specific to any challenge.
// -------------------------------------------------------------------------------//

use openssl::symm::{Cipher, Crypter, Mode};

const BASE64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Function to get a byte from a hex character.
///
/// Panics if the byte is not a valid hex character.
pub fn get_byte_from_hex(byte: &u8) -> u8 {
	match byte {
		b'0' => 0,
		b'1' => 1,
		b'2' => 2,
		b'3' => 3,
		b'4' => 4,
		b'5' => 5,
		b'6' => 6,
		b'7' => 7,
		b'8' => 8,
		b'9' => 9,
		b'a' | b'A' => 10,
		b'b' | b'B' => 11,
		b'c' | b'C' => 12,
		b'd' | b'D' => 13,
		b'e' | b'E' => 14,
		b'f' | b'F' => 15,
		_ => panic!("Invalid hex character: {}", byte),
	}
}

/// This function maps a slice of pairs of hex values into a Vec of their decimal counterparts.
///
/// For example, it turns b"49" into b"I", as b"49" is the hex value for the UTF-8 character b"I".
pub fn hex_array_to_byte_array(list: &[u8]) -> Vec<u8> {

	let mut text = list.to_vec();

	let mut y = 0;
	while y < list.len() {
		if list.get(y + 1).is_some() {
			text[y] = get_byte_from_hex(&list[y]);
			text[y + 1] = get_byte_from_hex(&list[y + 1]);
			text[y] <<= 4;
			text[y] |= text[y + 1];
			y += 2;
		}

		else {
			text[y] = get_byte_from_hex(&list[y]);
			text[y] <<= 4;
			break;
		}
	}

	text.iter().enumerate().filter(|x| x.0 % 2 == 0).map(|x| *x.1).collect::<Vec<u8>>()
}

/// Home-made function to decrypt data from base64.
pub fn decrypt_base64(encrypted_data: &[u8]) -> Vec<u8> {
	let mut encrypted_data = encrypted_data.to_vec();
	let mut decrypted_data = vec![];

	// Remove line feed, as those have to be ignored.
	encrypted_data.retain(|x| *x != b'\n');

	let mut index = 0;
	while index < encrypted_data.len() {

		if encrypted_data.get(index + 3).is_some() {
			let range = &encrypted_data[index..index + 4];
			let mut bits: u32 = 0;
			let mut padding_bytes = 0;

			for (cycle, value) in range.iter().enumerate() {
				if let Some(pos) = BASE64_TABLE.iter().position(|x| x == value) { bits |= pos as u32; }
				else if value == &61 { padding_bytes += 1 }

				if cycle != 3 { bits <<= 6; }
			}

			let mut x = vec![];
			let mut y = 0;
			if padding_bytes > 0 { bits >>= 8 * padding_bytes };
			while y < (3 - padding_bytes) {
				x.push((bits & 255) as u8);
				if y < 2 { bits >>= 8 };
				y += 1;
			}

			x.reverse();
			decrypted_data.append(&mut x);

			index += 4;
		}
		else { break; }
	}
	decrypted_data
}

/// Home-made function to encrypt data to base64.
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

/// Function to XOR two slices and return the XORed data.
///
/// Basically, this means it executes a XOR operation of one slice against the other one, and returns the result.
pub fn xor_data(data_1: &[u8], data_2: &[u8]) -> Vec<u8> {

	// We can only xor data slices of equal length, so we have to append zeros to the short one, if one is shorter.
	let mut data_1 = data_1.to_vec();
	let mut data_2 = data_2.to_vec();

	if data_1.len() != data_2.len() {
		if data_1.len() > data_2.len() {
			let diff = data_1.len() - data_2.len();
			data_2.extend(vec![0; diff].iter());
		} else {
			let diff = data_2.len() - data_1.len();
			data_1.extend(vec![0; diff].iter());
		}
	}

	let mut result = Vec::with_capacity(data_1.len());
	let zip = data_1.iter().zip(data_2.iter());
	zip.for_each(|(x, y)| result.push(x ^ y));
	result
}

/// Function to pad the provided vector, filling it with the amount of bytes to the end of the vector.
pub fn padd_to_end(block: &[u8], block_size: usize) -> Vec<u8> {
	if block.len() > block_size {
		panic!("Block is too big or block size too small");
	} else if block.len() < block_size {
		let mut data = block.to_vec();
		let bytes_to_padd = block_size - block.len();
		data.append(&mut vec![bytes_to_padd as u8; bytes_to_padd]);
		data
	} else {
		block.to_vec()
	}
}

/// Function to detect the hamming distance (amount of different bits) between two slices of data.
pub fn detect_hamming_distance(data_1: &[u8], data_2: &[u8]) -> u32 {
	let zip = data_1.iter().zip(data_2.iter());
	let hamming_distance = zip.fold(0, |acc, (x, y)| acc + (x ^ y).count_ones());
	hamming_distance
}

/// Function to detect the key size of a key used while XORing the provided data.
pub fn detect_fixed_xor_keysize(data: &[u8]) -> u32 {
	let mut key_size = 99999;
	let mut normal_big = 99999;

	for size in 2..40 {

		let mut y = 0;
		let mut normal = 0;
		while data.get(size * (y + 2)).is_some() {
			let range_1 = &data[(size * y)..(size * (y + 1))];
			let range_2 = &data[(size * (y + 1))..(size * (y + 2))];

			let hamming_distance = detect_hamming_distance(range_1, range_2);
			normal += hamming_distance / size as u32;
			y += 1;
		}

		normal /= (y / 2) as u32;
		if normal < normal_big { key_size = size as u32; normal_big = normal; }
	}

	key_size
}

/// Home-made function to encrypt a block of data in CBC mode, using the provided IV and key.
pub fn encrypt_aes_128_cbc(data: &[u8], iv: &[u8], key: &[u8]) -> Vec<u8> {

    // We know all blocks are the same length, but not their length, so we need to find that first.
    let key_size = key.len();
    let mut iv = iv.to_vec();
    let mut result = vec![];

	// CBC is basically XORed data (with IV or previous cyphertext) encrypted with ECB. So we do that.
    let cypher = Cipher::aes_128_ecb();
	let mut crypter = Crypter::new(cypher, Mode::Encrypt, key,None).unwrap();
	crypter.pad(false);

    for block in data.chunks(key_size as usize) {

    	// Padd the block before xoring it, just in case the block is smaller than we need.
    	let block_padded = padd_to_end(&block, key_size);
    	let block_xor = xor_data(&block_padded, &iv);

    	let mut block_cbc = vec![0; block_xor.len() + cypher.block_size()];
		crypter.update(&block_xor, &mut block_cbc).unwrap();
    	block_cbc.truncate(block_xor.len());

		iv = block_cbc.to_vec();
    	result.append(&mut block_cbc);
    }

    result
}

/// Home-made function to decrypt a block of data in CBC mode, using the provided IV and key.
pub fn decrypt_aes_128_cbc(encrypted_data: &[u8], iv: &[u8], key: &[u8]) -> Vec<u8> {
    let key_size = key.len();
    let mut iv = iv.to_vec();
    let mut result = vec![];

    let cypher = Cipher::aes_128_ecb();
	let mut crypter = Crypter::new(cypher, Mode::Decrypt, key,None).unwrap();
	crypter.pad(false);

	// Custom per-block implementation of CBC mode over ECB mode.
    for block in encrypted_data.chunks(key_size as usize) {
    	let mut block_xor = vec![0; block.len() + cypher.block_size()];
		crypter.update(&block, &mut block_xor).unwrap();
		block_xor.truncate(key_size);

    	let mut block_decrypted = xor_data(&block_xor, &iv);

    	// Detect and remove padding.
    	if let Some(last_byte) = block_decrypted.last() {
    		let last_byte = *last_byte as usize;
    		if last_byte < block_decrypted.len() {
    			if block_decrypted[block_decrypted.len() - last_byte..].iter().all(|x| *x as usize == last_byte) {
    				block_decrypted.truncate(block_decrypted.len() - last_byte);
    			}
    		}
    	}

    	result.append(&mut block_decrypted);
    	iv = block.to_vec();
    }

    result
}

/// This function scores strings according with character frequency in english, and returns the best scored one alongside its score.
pub fn score_strings_by_frequency<S: AsRef<str>>(strings: &[S]) -> (usize, String) {
	let mut most_scored = (0, String::new());
	let mut score = 0;
	for string in strings {
		let string = string.as_ref();
		score += string.matches("e").count() * 12;
		score += string.matches("t").count() * 11;
		score += string.matches("a").count() * 10;
		score += string.matches("o").count() * 9;
		score += string.matches("i").count() * 8;
		score += string.matches("n").count() * 7;
		score += string.matches("s").count() * 6;
		score += string.matches("h").count() * 5;
		score += string.matches("r").count() * 4;
		score += string.matches("d").count() * 3;
		score += string.matches("l").count() * 2;
		score += string.matches("u").count() * 1;
		if score > most_scored.0 { most_scored = (score, string.to_string()); }
		score = 0;
	}
	most_scored
}
