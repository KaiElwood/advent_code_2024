use std::{io::Read};

fn main() {
    // here i'll create a couple variables in a tuple to take the get file input
    let (mut l1, mut l2) = get_text();
    // here I need to sort the two lists from smallest to largest

    l1.sort();
    l2.sort();

    let mut total = 0;
    let mut similarity = 0;

    for i in 0..l1.len() {
        let sum = l1[i] - l2[i];
        total += sum.abs();
    }

    // now I need to find similarity score

    l1.iter().for_each(|val| {
        let num_occurances = l2.iter().filter(|x| *x == val).count() as i32;
        similarity += val * num_occurances;
    });
    println!("The similarity score is: {}", similarity);
    println!("The total is: {}", total);
}



fn get_text() -> (Vec<i32>, Vec<i32>) {
    // here i'm trying to do something and then expecting it to not do the thing. why? because it allows me to set a custom error message
    let mut file = std::fs::File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();
    contents.split("\n").for_each(|line| {
        let mut line = line.split("   ");
        if line.clone().next() == Some("") {
            return;
        }
        let val = line.next().unwrap();
        let val = val.trim().parse::<i32>().expect("couldn't parse val: {val}");
        vec1.push(val);
        let val = line.next().unwrap();
        let val = val.trim().parse::<i32>().expect("couldn't parse val: {val}");
        vec2.push(val);
    });
    return (vec1, vec2);
}
