extern crate phrases;
use phrases::english;
use phrases::korean::{greetings,farewells};

fn main() {
    println!("Hello in Korean : {}", greetings::hello());
    println!("Bye in Korean : {}", farewells::goodbye());
    println!("Hello in English : {}", english::hello());
    println!("Bye in English : {}", english::goodbye());
}
