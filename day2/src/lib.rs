pub use self::solution::*;

// Create a new module that contains all the actual implementation
mod solution {
    use std::io::Read;

    pub fn get_text() -> Vec<Vec<i32>> {
        let mut file = std::fs::File::open("input.txt").expect("file not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("something went wrong reading the file");

        let mut vec: Vec<Vec<i32>> = Vec::new();
        contents.split("\n").for_each(|line| {
            let line = line.split(" ");
            if line.clone().next() == Some("") {
                return;
            }
            let line = line.map(|val| val.trim().parse::<i32>().expect("couldn't parse val: {val}"));
            vec.push(line.collect());
        });
        vec
    }

    pub fn is_safe(report: &[i32]) -> bool {
        // Your existing is_safe implementation
        let mut increasing = true;
        for i in 0..report.len() - 1 {
            let diff = (report[i+1] - report[i]).abs();
            if diff == 0 {
                return false;
            };
            if i == 0 {
                if report[report.len() - 1] < report[i] {
                    increasing = false;
                }
            }
            if diff > 3 {
                return false
            }
            if increasing {
                if report[i+1] < report[i] {
                    return false;
                }
            } else {
                if report[i+1] > report[i] {
                    return false;
                }
            }
        }
        true
    }

    // runs in around 178 µs
    pub fn is_safe_dampened(report: &[i32]) -> bool {
        report.iter().enumerate().any(|(i, _)| {
            let mut new = report.to_owned();
            new.remove(i);
            is_safe(&new)
        })
    }

    // runs in around 310 µs
    pub fn is_safe_dampened_2(report: &[i32]) -> bool {
      match is_safe_with_el(&report) {
          SafeResult::Safe(true) => return true,
          SafeResult::Safe(false) => return false,
          SafeResult::Indices(i, j) => {
                  let mut remove_i_report = report.to_owned();
                  let mut remove_j_report = report.to_owned();
  
                  remove_i_report.remove(i as usize);
                  remove_j_report.remove(j as usize);
                  let result = [&remove_i_report, &remove_j_report].iter().any(|rep| {
                      is_safe(rep)
                  });
  
                  return result;
              }
      }
  }
  
  pub enum SafeResult {
      Safe(bool),
      Indices(i32, i32),
  }
  
  pub fn is_safe_with_el(report: &[i32]) -> SafeResult {
      let mut increasing = true;
      for i in 0..report.len() - 1 {
          // calculate the diff
          let diff = (report[i+1] - report[i]).abs();
  
          // if equal, then not safe
          if diff == 0 {
              return SafeResult::Indices(i as i32, (i + 1) as i32);
          };
          // when i is zero, figure out whether it is increasing or decreasing
          if i == 0 {
              if report[report.len() - 1] < report[i] {
                  increasing = false;
              }
          }
  
          // if the diff is greater than 3, then not safe
          if diff > 3 {
              return SafeResult::Indices(i as i32, (i + 1) as i32);
          }
          
          // if increasing, then the next number must be higher. if decreasing, then the next num must be lower
          if increasing {
              if report[i+1] < report[i] {
                  return SafeResult::Indices(i as i32, (i + 1) as i32);
              }
          } else {
              if report[i+1] > report[i] {
                  return SafeResult::Indices(i as i32, (i + 1) as i32);
              }
          }
      }
      return SafeResult::Safe(true);
  }
}