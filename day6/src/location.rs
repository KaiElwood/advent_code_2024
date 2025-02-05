use crate::{error::MoveError, direction::Direction};

#[derive(Debug, Clone, Copy , PartialEq, Eq, Hash)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PossibleLocation {
  pub x: i32,
  pub y: i32,
}

impl TryFrom<PossibleLocation> for Location {
  type Error = MoveError;
  fn try_from(pos_loc: PossibleLocation) -> Result<Location, Self::Error> {
    if pos_loc.x < 0 || pos_loc.y < 0 {
      return Err(MoveError::OutOfBounds);
    }
    Ok(Location {
      x: pos_loc.x as usize,
      y: pos_loc.y as usize,
    })
  }
}

impl Location {
  pub fn move_in(&self, direction: Direction) -> PossibleLocation {
    PossibleLocation {
      x: self.x as i32 + direction.dx(),
      y: self.y as i32 + direction.dy(),
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_in() {
        let loc = Location { x: 0, y: 0 };
        assert_eq!(loc.move_in(Direction::Right), PossibleLocation { x: 1, y: 0 });
        assert_eq!(loc.move_in(Direction::Down), PossibleLocation { x: 0, y: 1 });
        assert_eq!(loc.move_in(Direction::Left), PossibleLocation { x: -1, y: 0 });
        assert_eq!(loc.move_in(Direction::Up), PossibleLocation { x: 0, y: -1 });
    }
}