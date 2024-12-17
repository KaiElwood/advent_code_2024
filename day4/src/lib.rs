// --- Day 4: Ceres Search ---
// "Looks like the Chief's not here. Next!" One of The Historians pulls out a device and pushes the only button on it. After a brief flash, you recognize the interior of the Ceres monitoring station!

// As the search for the Chief continues, a small Elf who lives on the station tugs on your shirt; she'd like to know if you could help her with her word search (your puzzle input). She only has to find one word: XMAS.

// This word search allows words to be horizontal, vertical, diagonal, written backwards, or even overlapping other words. It's a little unusual, though, as you don't merely need to find one instance of XMAS - you need to find all of them. Here are a few ways XMAS might appear, where irrelevant characters have been replaced with .:


// ..X...
// .SAMX.
// .A..A.
// XMAS.S
// .X....
// The actual word search will be full of letters instead. For example:

// MMMSXXMASM
// MSAMXMSMSA
// AMXSXMAAMM
// MSAMASMSMX
// XMASAMXAMM
// XXAMMXXAMA
// SMSMSASXSS
// SAXAMASAAA
// MAMMMXMMMM
// MXMXAXMASX
// In this word search, XMAS occurs a total of 18 times; here's the same word search again, but where letters not involved in any XMAS have been replaced with .:

// ....XXMAS.
// .SAMXMS...
// ...S..A...
// ..A.A.MS.X
// XMASAMX.MM
// X.....XA.A
// S.S.S.S.SS
// .A.A.A.A.A
// ..M.M.M.MM
// .X.X.XMASX
// Take a look at the little Elf's word search. How many times does XMAS appear?

// one method would be to create multiple arrays of information
// one array for each direction
// then loop through each array forwards and backwards
// how do I do this?

// or I could just figure out what the patterns are and then test them as I loop through...
// no, that seems like a lot of work
// the ebst way is to crete the arrays and then loop through them

use regex::Regex;
use std::collections::HashMap;

pub fn pt2(text: &str) {
    let text = text.to_string();
    let mut map: HashMap<usize, usize> = HashMap::new();
    let arrays: Vec<Vec<(usize, char)>> = parse_text(&text)[280..].to_vec();
    let total = arrays.iter().map(|array| {
        count_xmas_2(array, &mut map)
    }).sum::<usize>();
    // println!("The total is: {}", total);
}

pub fn pt1(text: &str) {
    let text = text.to_string();
    let arrays = parse_text(&text);
    let total: usize = arrays.into_iter()
        .map(|array| array.into_iter().map(|(_, c)| c).collect::<String>())
        .map(|array| count_xmas(array))
        .sum::<usize>();
    // println!("The total is: {}", total);
}

pub fn get_text() -> String {
    let file_path = "input.txt";
    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    return contents;
}

fn parse_text(text: &String) -> Vec<Vec<(usize, char)>> {
    let mut vec: Vec<Vec<(usize, char)>> = Vec::new();
    let lines = text.split("\n").collect::<Vec<&str>>();
    let lines_count = lines.iter().count();
    let rows = text.split("\n").next().unwrap().len();
    for _ in 0..(6*lines_count) {
        vec.push(Vec::new());
    };

    text.split("\n").enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {

            if j == 0 {
                line.chars().enumerate().for_each(|(it, c)| {
                    vec[i].push((i * lines_count + it, c));
                });
            }
            vec[lines_count+j].push((i * lines_count + j, c));
            vec[(2*lines_count)+1+i+j].push((i * lines_count + j, c));
        });
    });


    for i in 0..lines_count {
        let rev_line_count = lines_count - i - 1;
        let line = lines[rev_line_count];
        line.chars().enumerate().for_each(|(j, c)| {
            vec[4*lines_count+i+j].push((rev_line_count * lines_count + j, c));
        });
    }
    vec
}

fn count_xmas(array: String) -> usize {
    let xmas = Regex::new(r"XMAS").unwrap();
    let samx = Regex::new(r"SAMX").unwrap();
    let xmas_total = xmas.captures_iter(&array).count();
    let samx_total = samx.captures_iter(&array).count();
    xmas_total + samx_total
}

fn count_xmas_2(array: &Vec<(usize, char)>, map: &mut HashMap<usize, usize>) -> usize {
    let mut total = 0;
    let str = array.iter().map(|(_, c)| c).collect::<String>();
    let mas = Regex::new(r"MAS").unwrap();
    let sam = Regex::new(r"SAM").unwrap();
    for cap in mas.captures_iter(&str) {
        let pos = cap.get(0).unwrap().start() + 1;
        let arr_pos = array[pos].0;
        let count = map.get(&arr_pos).unwrap_or(&0);
        if map.get(&arr_pos).is_none() {
            map.insert(arr_pos, 1);
        } else {
            map.insert(arr_pos, count + 1);
            total += 1;
        }
    }
    for cap in sam.captures_iter(&str) {
        let pos = cap.get(0).unwrap().start() + 1;
        let arr_pos = array[pos].0;
        let count = map.get(&arr_pos).unwrap_or(&0);
        if map.get(&arr_pos).is_none() {
            map.insert(arr_pos, 1);
        } else {
            map.insert(arr_pos, count + 1);
            total += 1;
        };
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        // abc
        // def
        // ghi
        let text = "abc\ndef\nghi";
        let rows = parse_text(&text.to_string());
        print_grid(&rows);
        // print_grid(&rotate_n(rows.clone(), 1));
        // print_grid(&rotate_n(rows.clone(), 2));
        // print_grid(&rotate_n(rows.clone(), 3));
        // assert_eq!(rotate_n(rows.clone(), 0), rows);
        // assert_eq!(rotate_n(rows.clone(), 4), rows);
        // assert_eq!(rotate_n(rows.clone(), 1), ["gda", "heb", "ifc"]);
        // assert_eq!(rotate_n(rows.clone(), 2), ["ihg", "fed", "cba"]);
        // assert_eq!(rotate_n(rows.clone(), 3), ["cfi", "beh", "adg"]);
    }

    // #[test]
    // fn test_rotate_diag() {
    //     let rows = ["abc", "def", "ghi"];
    //     let rows: Vec<String> = rows.iter().map(|s| s.to_string()).collect();
    //     let rotated = ["..a", ".db", "gec", "hf.", "i.."];
    //     print_grid(&rows);
    //     print_grid(&rotated.iter().map(|s| s.to_string()).collect());
    //     print_grid(&rotate_diag(rows.clone()));
    //     print_grid(&rotate_diag(rotate_diag(rows.clone())));
    //     assert_eq!(rotate_diag(rows.clone()), rotated);
    // }

    // #[test]
    // fn test_reverse_diag() {
    //     let rows = ["abc", "def", "ghi"];
    //     let rows: Vec<String> = rows.iter().map(|s| s.to_string()).collect();
    //     let rotated = ["..c", ".bf", "aei", "dh.", "g.."];
    //     print_grid(&rows);
    //     print_grid(&rotated.iter().map(|s| s.to_string()).collect());
    //     print_grid(&rotate_diag(rows.clone()));
    //     assert_eq!(rotate_diag(rotate_n(rows.clone(), 3)), rotated);
    // }

    // #[test]
    // fn test_all() {
    //     let rows = ["abc", "def", "ghi"];
    //     let rows = rows.iter().map(|s| s.to_string()).collect();
    //     let grids = get_rotations(rows);
    //     let rows: Vec<&String> = grids.iter().flat_map(|g| g.iter()).collect();
    //     let expected_strs = ["abc", "cba", "gec", "aei", "gda", "adg"];
    //     for s in expected_strs {
    //         assert!(rows.contains(&&s.to_string()), "s: {}", s);
    //         assert!(
    //             rows.contains(&&s.chars().rev().collect::<String>()),
    //             "s: {}",
    //             s
    //         );
    //     }
    // }

    fn print_grid(rows: &Vec<Vec<(usize, char)>>) {
        println!("Printing grid:");
        rows.iter().for_each(|row| {
            row.iter().for_each(|(i, c)| {
                println!("pos: {}, char: {}", i, c);
            });
            println!();
        });
        // for (i, r) in rows {
        //     println!("pos: {}, char: {}", i, r);
        // }
        println!();
    }
}
