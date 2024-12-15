use day2::{get_text, is_safe, is_safe_dampened, is_safe_dampened_2};

fn main() {
    let reports: Vec<Vec<i32>> = get_text();

    let reports_final = reports.iter().filter(|report| is_safe(report)).count();

    let reports_final_dampened = reports.iter().filter(|report| is_safe_dampened_2(report)).count();

    // let dampened_reports = get_dampened_reports(reports);

    println!("The number of safe reports is: {}", reports_final);
    println!("The number of dampened safe reports is: {}", reports_final_dampened);
}