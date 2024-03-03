use std::io::{self, Read};

fn main() {
    println!("Hello! and welcome to Oxyd text editor!");

    for c in io::stdin().bytes() {
        let c = c.unwrap() as char;

        if c == 'q' {
            break;
        }

        print!("{c}");
    }

    println!("Quit!");
}
