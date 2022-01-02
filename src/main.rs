pub mod utils;
pub mod set_1;
pub mod set_2;

#[cfg(test)]
pub mod tests;

/// I'm too lazy to write the same thing again and again...
macro_rules! set {
    ($set:expr) => {
        println!("");
        println!("");
        println!("-------------------------------------------------------------------------------------");
        println!("---- Set {}", $set);
        println!("-------------------------------------------------------------------------------------");
    };
}

/// Same as with the set one...
macro_rules! challenge {
    ($set:ident, $challenge:ident, $challenge_id:expr) => {
        println!("-- Challenge {}", $challenge_id);
        println!("");
        $set::$challenge::challenge();
        println!("");
        println!("-------------------------------------------------------------------------------------");
    };
}

fn main() {
    set!("1");
    challenge!(set_1, challenge_1, "1");
    challenge!(set_1, challenge_2, "2");
    challenge!(set_1, challenge_3, "3");
    challenge!(set_1, challenge_4, "4");
    challenge!(set_1, challenge_5, "5");
    challenge!(set_1, challenge_6, "6");
    challenge!(set_1, challenge_7, "7");
    challenge!(set_1, challenge_8, "8");

    set!("2");
    challenge!(set_2, challenge_9, "9");
    challenge!(set_2, challenge_10, "10");
}
