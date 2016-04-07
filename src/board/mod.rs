pub use self::orientation::Orientation;
pub use self::orientation::Coordinate;
pub use self::board::Board;
pub use self::shot::Hit;
pub use self::shot::HitStatus;

mod orientation;
mod board;
pub mod shot;
#[cfg(test)]
mod tests;
