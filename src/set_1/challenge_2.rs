use utils::get_byte_from_hex;

pub fn challenge() {
	let string_1 = b"1c0111001f010100061a024b53535009181c";
	let string_2 = b"686974207468652062756c6c277320657965";
	let intended_result = b"746865206b696420646f6e277420706c6179";
	let intended_result = intended_result.iter().map(|x| *get_byte_from_hex(x) & 15).collect::<Vec<u8>>();
	let mut result = vec![];
	let zip = string_1.iter().zip(string_2.iter());

	zip.for_each(|(x, y)| result.push((*get_byte_from_hex(&x) & 15) ^ (*get_byte_from_hex(&y) & 15)));

    assert_eq!(result, intended_result.to_vec());
}
