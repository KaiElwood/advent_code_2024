
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::{error::MoveError, guard::Guard, location::{Location, PossibleLocation}};

#[derive(Clone)]
pub struct Map {
  data: HashMap<Location, char>,
  width: usize,
  height: usize,
}

impl From<&mut File> for Map {
  fn from(file: &mut File) -> Map {
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    Map::from(contents)
  }
}

impl From<String> for Map {
  fn from(contents: String) -> Map {
    let rows: Vec<&str> = contents.split("\n").collect();
    let data = rows
      .iter()
      .enumerate()
      .flat_map(|(y, row)| {
        row.chars()
          .enumerate()
          .map(move |(x, c)| (Location { x, y }, c))
          .collect::<Vec<(Location, char)>>()
      })
      .collect();

    let width = rows[0].len();
    let height = rows.len();
    
    Map { data, width, height }
  }
}

impl Map {
  pub fn find_guard(&self) -> Option<Location> {
    self.data
      .iter()
      .filter(|(_, c)| Guard::is_guard(c))
      .map(|(loc, _)| loc)
      .next()
      .copied()
  }

  pub fn check_in_bounds(&self, loc: &PossibleLocation) -> Result<(), MoveError> {
    if loc.x < 0 || loc.y < 0 {
      return Err(MoveError::OutOfBounds);
    }
    if loc.x >= self.width as i32 || loc.y >= self.height as i32 {
      return Err(MoveError::OutOfBounds);
    }
    Ok(())
  }

  pub fn try_char_at(&self, loc: PossibleLocation) -> Result<char, MoveError> {
    self.check_in_bounds(&loc)?;
    Ok(self.char_at(&loc.try_into()?))
  }

  pub fn set_char_at(&mut self, loc: Location, c: char) {
    self.data.insert(loc, c);
  }

  pub fn char_at(&self, loc: &Location) -> char {
    *self.data.get(loc).unwrap()
  }

  pub fn height(&self) -> usize {
    self.height
  }

  pub fn width(&self) -> usize {
    self.width
  }
}

#[cfg(test)]
mod tests {
    use crate::direction::Direction;

    use super::*;

    #[test]
    fn in_bounds() {
        let map = Map::from("abc\ndef".to_string());
        assert!(map.check_in_bounds(&PossibleLocation { x: 0, y: 0 }).is_ok());
        assert!(map.check_in_bounds(&PossibleLocation { x: 2, y: 0 }).is_ok());
        assert!(map.check_in_bounds(&PossibleLocation { x: 0, y: 1 }).is_ok());
        assert!(map.check_in_bounds(&PossibleLocation { x: 2, y: 1 }).is_ok());
    }

    #[test]
    fn out_of_bounds() {
        let map = Map::from("abc\ndef".to_string());
        let cases = [
            (PossibleLocation { x: -1, y: 0 }, Direction::Left),
            (PossibleLocation { x: 0, y: -1 }, Direction::Up),
            (PossibleLocation { x: 3, y: 0 }, Direction::Right),
            (PossibleLocation { x: 0, y: 3 }, Direction::Down),
        ];
        for (loc, _dir) in cases {
            assert_eq!(
                map.check_in_bounds(&loc),
                Err(MoveError::OutOfBounds)
            );
        }
    }

    #[test]
    fn find_guard() {
        let map = Map::from("abc\ndef\nghi".to_string());
        assert_eq!(map.find_guard(), None);

        let map = Map::from("abc\ndef\n.>i".to_string());
        assert_eq!(map.find_guard(), Some(Location { x: 1, y: 2 }));

        // undefined behavior now that we're storing chars by location in a hashmap
        let map = Map::from("^bc\ndef\n<gi".to_string());
        let guard_loc = map.find_guard().unwrap();
        assert!(
            guard_loc == Location { x: 0, y: 0 }
                || guard_loc == Location { x: 0, y: 2 }
        );
    }

    #[test]
    fn char_at() {
        let map = Map::from("abc\ndef\nghi".to_string());
        let cases = [
            (Location { x: 0, y: 0 }, 'a'),
            (Location { x: 1, y: 0 }, 'b'),
            (Location { x: 2, y: 0 }, 'c'),
            (Location { x: 0, y: 1 }, 'd'),
            (Location { x: 1, y: 1 }, 'e'),
            (Location { x: 2, y: 1 }, 'f'),
            (Location { x: 0, y: 2 }, 'g'),
            (Location { x: 1, y: 2 }, 'h'),
            (Location { x: 2, y: 2 }, 'i'),
        ];
        for (loc, c) in cases {
            assert_eq!(map.char_at(&loc), c);
        }
    }
}