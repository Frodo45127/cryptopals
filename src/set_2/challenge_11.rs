// -------------------------------------------------------------------------------//
// Cryptopals, Set 2, Challenge 11: https://cryptopals.com/sets/2/challenges/11
// Impl by Frodo45127
// -------------------------------------------------------------------------------//

use crate::utils::*;

const DATA: &[u8; 164] = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

pub fn challenge() {

    // So, we're not told any specific data about what should we encrypt... so we just encrypt the same letter again and again.
    // This will generate similar blocks when ECB encryption is used, allowing us to identify them.
    let (result, cyphers) = encrypt_aes_128_ecb_and_cbc(DATA, );
    let (ok, err) = encryption_oracle(&result, &cyphers);

    println!("Results Ok: {}", ok);
    println!("Results Err: {}", err);

    // This can easely fail, so we are ok with at least 50% success.
    assert!(ok > err);

    println!("Challenge passed!!!");
}
