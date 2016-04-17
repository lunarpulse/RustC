extern crate phrases as sayings;

use sayings::english::greetings as en_greetings;
use sayings::english::farewells::*;
use sayings::korean::{self, greetings as ko_greetings,farewells as ko_farewells};

fn main() {
    println!("Hello in Korean : {}", ko_greetings::hello());
    println!("Bye in Korean : {}", ko_farewells::goodbye());
    print!("Hello in English : {} ", en_greetings::hello());
    println!("and in Korean : {}", korean::greetings::hello());
    println!("Bye in English : {}", goodbye());
}
