mod grid;
mod board;
mod pieces;

fn main() {
    println!("Hello, world!");
    let b = board::Board::new(5);
    println!("{}", b);
}
