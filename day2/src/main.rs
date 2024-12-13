use std::io::Read;

// While the Red-Nosed Reindeer nuclear fusion/fission plant appears to contain no sign of the Chief Historian, the engineers there run up to you as soon as they see you. Apparently, they still talk about the time Rudolph was saved through molecular synthesis from a single electron.

// They're quick to add that - since you're already here - they'd really appreciate your help analyzing some unusual data from the Red-Nosed reactor. You turn to check if The Historians are waiting for you, but they seem to have already divided into groups that are currently searching every corner of the facility. You offer to help with the unusual data.

// The unusual data (your puzzle input) consists of many reports, one report per line. Each report is a list of numbers called levels that are separated by spaces. For example:

// 7 6 4 2 1
// 1 2 7 8 9
// 9 7 6 2 1
// 1 3 2 4 5
// 8 6 4 4 1
// 1 3 6 7 9
// This example data contains six reports each containing five levels.

// The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety systems can only tolerate levels that are either gradually increasing or gradually decreasing. So, a report only counts as safe if both of the following are true:

// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.
// In the example above, the reports can be found safe or unsafe by checking those rules:

// 7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
// 1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
// 9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
// 1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
// 8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
// 1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
// So, in this example, 2 reports are safe.

// Analyze the unusual data from the engineers. How many reports are safe?

fn main() {
    let mut reports: Vec<Vec<i32>> = get_text();
    let mut numSafe = 0;

    reports.iter().for_each(|report| {
        // what is the fastest way to check all of this? I could do a quick map... I think the best option is just to foreach throuhg all of it, so we wind up with o(n2)
        let mut increasing = true;
        let mut safe = true;
        for i in 0..report.len() - 1 {
            // calculate the diff
            let diff = (report[i+1] - report[i]).abs();

            // if equal, then not safe
            if (diff == 0) {
                safe = false;
                break;
            };

            // match increasing {
            //     None => increasing = Some(diff > 0);
            //     Some(true) => {
            //         if diff < 0 {
            //             return false;
            //         }
            //     }
            //     Some(false) => {
            //         if diff > 0 {
            //             return false;
            //         }
            //     }
            // }

            // when i is zero, figure out whether it is increasing or decreasing
            if (i == 0) {
                if (report[i+1] < report[i]) {
                    increasing = false;
                }
            }

            // if the diff is greater than 3, then not safe
            if (diff > 3) {
                safe = false;
                break;
            }
            
            // if increasing, then the next number must be higher. if decreasing, then the next num must be lower
            if (increasing) {
                if (report[i+1] < report[i]) {
                    safe = false;
                    break;
                }
            } else {
                if (report[i+1] > report[i]) {
                    safe = false;
                    break;
                }
            }
        }
        if (safe) {
            numSafe += 1;
        }
    });

    println!("The number of safe reports is: {}", numSafe);
}

fn get_text() -> Vec<Vec<i32>> {
    let mut file = std::fs::File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");

    let mut vec: Vec<Vec<i32>> = Vec::new();
    contents.split("\n").for_each(|line| {
        let mut line = line.split(" ");
        if line.clone().next() == Some("") {
            return;
        }
        let line = line.map(|val| val.trim().parse::<i32>().expect("couldn't parse val: {val}"));
        vec.push(line.collect());
    });
    vec
}
