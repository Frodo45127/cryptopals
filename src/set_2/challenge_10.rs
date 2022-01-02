// -------------------------------------------------------------------------------//
// Cryptopals, Set 2, Challenge 10: https://cryptopals.com/sets/2/challenges/10
// Impl by Frodo45127
// -------------------------------------------------------------------------------//

use crate::utils::*;

use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;

const KEY: &[u8; 16] = b"YELLOW SUBMARINE";

const IV: &[u8; 16] = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

pub fn challenge() {
    let mut file = BufReader::new(File::open(PathBuf::from("assets/2-10")).unwrap());
    let mut data_to_decrypt = vec![];
    file.read_to_end(&mut data_to_decrypt).unwrap();

    let data_decrypted_base64 = decrypt_base64(&data_to_decrypt);

    let result = decrypt_aes_128_cbc(&data_decrypted_base64, IV, KEY);
    let string = String::from_utf8(result).unwrap();

    println!("Decrypted Text: \n {}", string);
}
