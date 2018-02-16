// Language:       Rust
// Web site:       http://www.rust-lang.org/
// Last tested on: Ubuntu 17.10
// Requires:       apt-get install rustc
fn main() {
    for i in 1..101 {
        if      i % 15 == 0 { println!("FizzBuzz") }
        else if i %  3 == 0 { println!("Fizz") }
        else if i %  5 == 0 { println!("Buzz") }
        else                { println!("{}", i) }
    }
}
