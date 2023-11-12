mod d1a;
mod d1b;
mod d2a;
mod d2b;
mod d3a;
mod d3b;
mod d4a;
mod d4b;

use std::io;

fn main() -> io::Result<()> {
    match d4b::d4b() {
        Ok(_) => println!("Success!"),
        Err(_) => println!("Execution failure."),
    }
    Ok(())
}
