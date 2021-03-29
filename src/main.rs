pub mod utils;
pub mod set_1;

const BASE64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn main() {
    println!("-------------------------------------------------------------------------------------");
    println!("Set 1");
    println!("-------------------------------------------------------------------------------------");
    println!("Challenge 1");
    set_1::challenge_1::challenge();
    println!("-------------------------------------------------------------------------------------");
    println!("Challenge 2");
    set_1::challenge_2::challenge();
    println!("-------------------------------------------------------------------------------------");
    println!("Challenge 3");
    set_1::challenge_3::challenge();
    println!("-------------------------------------------------------------------------------------");
    println!("Challenge 4");
    set_1::challenge_4::challenge();
    println!("-------------------------------------------------------------------------------------");
    println!("Challenge 5");
    set_1::challenge_5::challenge();
    println!("-------------------------------------------------------------------------------------");
    println!("Challenge 6");
    set_1::challenge_6::challenge();
    println!("-------------------------------------------------------------------------------------");
    println!("Challenge 7");
    set_1::challenge_7::challenge();
    println!("-------------------------------------------------------------------------------------");
    println!("Challenge 8");
    set_1::challenge_8::challenge();
}
