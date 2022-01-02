// -------------------------------------------------------------------------------//
// Cryptopals, Set 1, Challenge 7: https://cryptopals.com/sets/1/challenges/7
// Impl by Frodo45127
// -------------------------------------------------------------------------------//

use crate::utils::*;

use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;

const KEY: &[u8; 16] = b"YELLOW SUBMARINE";

pub fn challenge() {

    // We're just asked to wire ECB decoding up and we are allowed to use openssl, so....
    let mut file = BufReader::new(File::open(PathBuf::from("assets/1-7")).unwrap());
    let mut data_to_decrypt = vec![];
    file.read_to_end(&mut data_to_decrypt).unwrap();

    let data_decrypted_base64 = decrypt_base64(&data_to_decrypt);

    let result = decrypt_aes_128_ecb(&data_decrypted_base64, KEY);
    let string = String::from_utf8(result).unwrap();

    println!("Decrypted Text: \n {}", string);
}
