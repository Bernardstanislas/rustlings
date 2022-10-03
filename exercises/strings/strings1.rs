// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

use std::str::FromStr;

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    String::from_str("blue").unwrap()
}
