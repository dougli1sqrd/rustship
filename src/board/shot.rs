use std::fmt;

use board::Coordinate;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum HitStatus {
    Hit,
    Miss
}

impl fmt::Debug for HitStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &HitStatus::Hit => write!(f, "Hit"),
            &HitStatus::Miss => write!(f, "Miss")
        }
    }
}

pub trait Hit {
    fn hits(&self, coordinate: Coordinate) -> HitStatus;

    fn resolve_hit(& mut self, coordinate: Coordinate);

    #[allow(unused_variables)]
    fn resolve_miss(& mut self, coordinate: Coordinate) {
        // The default implementation is to do nothing when we miss. Implementations of this trait
        // may want to do more than nothing.
    }

    fn receive_shot(& mut self, coordinate: Coordinate) -> HitStatus {
        let hit_status = self.hits(coordinate);
        match hit_status {
            HitStatus::Hit => self.resolve_hit(coordinate),
            HitStatus::Miss => self.resolve_miss(coordinate)
        };
        return hit_status;
    }
}
