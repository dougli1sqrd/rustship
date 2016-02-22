mod grid;
mod board;
mod pieces;

fn main() {
    println!("Hello, world!");
    let mut b = board::Board::new(5);
    println!("{}", b);
    b.place_ship(pieces::Ship::new(pieces::ShipType::Destroyer));
    println!("{}", b);
    println!("{:?}", pieces::Ship::new(pieces::ShipType::Destroyer));
}
