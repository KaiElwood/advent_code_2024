use std::fs::File;
use crate::{error::MoveError, guard::Guard, location::Location, map::Map};

// I need a function that creates finds the loops

// I need a function that reads in the data and creates a map

// I need a function that places an obstacle in each possible position and checks if a loop is created

// to see if loop is created we can throw an error if guard hits same spot in same direction

pub fn solve() -> usize {
  let mut file = File::open("input.txt").expect("Could not open file");
  let mut map = Map::from(&mut file);
  print!("Checking loops for map of size {}x{}", map.width(), map.height());
  let locs = get_all_locs_that_cause_loop(&mut map);
  println!("Found: {:?}", locs);
  locs.len()
}

fn get_all_locs_that_cause_loop(map: &mut Map) -> Vec<Location> {
  // let mut res = Vec::new();
  (0..map.height())
    .flat_map(|y| {
      (0..map.width())
        .map(|x| Location { x, y })
        .filter(|loc| loc_causes_loop(loc, map))
        .collect::<Vec<Location>>()
    })
    .collect()

  // print!("Checking loops for map of size {}x{}", map.width(), map.height());
  // for y in 0..map.height() {
  //   for x in 0..map.width() {
  //     let loc = Location { x, y };
  //     if loc_causes_loop(&loc, map) {
  //       res.push(loc);
  //     }
  //   }
  // }
  // res
}

fn loc_causes_loop(loc: &Location, map: &mut Map) -> bool {
  let c = map.char_at(loc);

  if Guard::is_guard(&c) || c == '#' {
    return false;
  }
  map.set_char_at(*loc, '#');
  let res;
  let mut guard = Guard::from(&*map);
  loop {
    match guard.step() {
      Ok(()) => (),
      Err(MoveError::StuckInLoop(_)) => {
        res = true;
        break;
      }
      Err(_) => {
        res = false;
        break;
      }
    }
  }
  map.set_char_at(*loc, c);
  res
}