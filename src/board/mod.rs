pub use self::orientation::Orientation;
pub use self::orientation::Coordinate;
pub use self::board::Board;

mod orientation;
mod board;
mod shot;
#[cfg(test)]
mod tests;
