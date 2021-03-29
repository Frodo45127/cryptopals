// -------------------------------------------------------------------------------//
// Cryptopals, Set 1, Challenge 8: https://cryptopals.com/sets/1/challenges/8
// Impl by Frodo45127
// -------------------------------------------------------------------------------//

use crate::utils::decrypt_base64;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::PathBuf;

pub fn challenge() {
    let file = BufReader::new(File::open(PathBuf::from("assets/1-8")).unwrap());

    // Logic here:
    // - Each 16 bytes is a block.
    // - Each block encoded with the same key will always result in the same output.
    // - The encoded data may have sections of bytes that repeat themselfs.
    // - So we try to find a row with repeated blocks.
    for text in file.split(b'\n') {
        let mut texts = vec![];
        let data_decrypted_base64 = decrypt_base64(&text.unwrap());
        for (row, text) in data_decrypted_base64.chunks(16).enumerate() {
            if texts.contains(&text) {
                return println!("Row: {}", row);
            } else {
                texts.push(text);
            }
        }
    }

    println!("Row not found");
}
