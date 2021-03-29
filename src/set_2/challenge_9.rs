// -------------------------------------------------------------------------------//
// Cryptopals, Set 2, Challenge 9: https://cryptopals.com/sets/2/challenges/9
// Impl by Frodo45127
// -------------------------------------------------------------------------------//

use crate::utils::padd_to_end;

pub fn challenge() {
    let source_data = b"YELLOW SUBMARINE";
    let target_data = b"YELLOW SUBMARINE\x04\x04\x04\x04";

    let result = padd_to_end(source_data, 20);
    assert_eq!(target_data.to_vec(), result);
}
