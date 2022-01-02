// -------------------------------------------------------------------------------//
// Module to write tests for the challenge helper functions.
// Impl by Frodo45127
// -------------------------------------------------------------------------------//

use crate::utils::*;

#[test]
/// Test to ensure our function to calculate the hamming distance is correct.
fn test_detect_hamming_distance() {

    let string_1 = b"this is a test";
    let string_2 = b"wokka wokka!!!";

    let hamming_distance = detect_hamming_distance(string_1, string_2);
    assert_eq!(hamming_distance, 37);
}

#[test]
/// Test to ensure our custom cbc encryption implementation works correctly.
fn test_encrypt_decrypt_aes_128_cbc() {
    let key: &[u8; 16] = b"YELLOW SUBMARINE";
    let iv: &[u8; 16] = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

    let plaintext = b"1234567890qwertyuio".to_vec();
    let encrypted = encrypt_aes_128_cbc(&plaintext, iv, key);
    let decrypted = decrypt_aes_128_cbc(&encrypted, iv, key);

    assert_eq!(plaintext, decrypted);
}
