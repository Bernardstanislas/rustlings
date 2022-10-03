// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

trait WithLen {
    fn len(self) -> usize;
}

impl WithLen for u8 {
    fn len(self) -> usize {
        100
    }
}

fn main() {
    let a: u8 = 3;

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
