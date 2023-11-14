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
mod d6a;
mod d6b;
mod d7a;
mod d7b;
mod d8a;
mod d8b;

use std::io;

fn main() -> io::Result<()> {
    match d8b::d8b() {
        Ok(_) => println!("Success!"),
        Err(_) => println!("Execution failure."),
    }
    Ok(())
}
