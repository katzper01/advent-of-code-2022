mod d1a;
mod d1b;
mod d2a;
mod d2b;
mod d3a;
mod d3b;
mod d4a;
mod d4b;
mod d5a;
mod d5b;

use std::io;

fn main() -> io::Result<()> {
    match d5b::d5b() {
        Ok(_) => println!("Success!"),
        Err(_) => println!("Execution failure."),
    }
    Ok(())
}
