use std::fs::File;
use crate::map::Map;
use crate::guard::Guard;

mod map;
mod location;
mod guard;
mod direction;
mod error;
mod obstruction;

fn main() {
    let map = load_map();
    let mut guard: Guard = Guard::from(&map);

    loop {
        if let Err(e) = guard.step() {
            println!("Went until error: {:?}", e);
            break;
        }
    }

    println!("Part 1: {}", guard.num_locations_visited());
    println!("Part 2: {}", obstruction::solve());
    // so basically here I can reuse what was already made in order to find spots where a loop is created. 
    // what defines a loop? I already have it coded
    // if a guard returns to the same spot and direction, then it's a loop
}

fn load_map() -> Map {
    let mut file = File::open("input.txt").expect("Could not open file");
    Map::from(&mut file)
}

// so basically I need to predict what positions will be walked on by the guard

// one method of doing this might be using a backtracking algorithm
// That's a lot of squares to backtrack on! Yes... but maybe fastest option
// no, a better way is just to create a grid, then create a method for "walking" forward

// so, create the vector grid and then go forward until guard is out of bounds
