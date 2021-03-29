// -------------------------------------------------------------------------------//
// Cryptopals, Set 1, Challenge 7: https://cryptopals.com/sets/1/challenges/7
// Impl by Frodo45127
// -------------------------------------------------------------------------------//

use crate::utils::decrypt_base64;

use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;

use openssl::symm::{Cipher, decrypt};

const KEY: &[u8; 16] = b"YELLOW SUBMARINE";

pub fn challenge() {

    // We're just asked to wire ECB decoding up and we are allowed to use openssl, so....
    let mut file = BufReader::new(File::open(PathBuf::from("assets/1-7")).unwrap());
    let mut data_to_decrypt = vec![];
    file.read_to_end(&mut data_to_decrypt).unwrap();

    let data_decrypted_base64 = decrypt_base64(&data_to_decrypt);

    let cypher = Cipher::aes_128_ecb();
    let result = decrypt(cypher, KEY, None, &data_decrypted_base64).unwrap();
    let string = String::from_utf8(result).unwrap();

    println!("Decrypted Text: \n {}", string);
}
