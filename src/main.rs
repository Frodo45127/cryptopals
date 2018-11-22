pub mod utils;
pub mod set_1;

const BASE64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn main() {
    set_1::challenge_1::challenge();
    set_1::challenge_2::challenge();
    set_1::challenge_3::challenge();
    set_1::challenge_4::challenge();
    set_1::challenge_5::challenge();
}
