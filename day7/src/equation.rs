
use crate::error::CalcError;

#[derive(Debug, PartialEq)]
pub struct Equation {
  pub is_solved: bool,
  parts: Vec<i32>,
  operators: Vec<Operator>,
  pub result: i64,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
  Multiply,
  Add,
  Concat,
}

pub trait Create<P, R> {
  fn create(parts: P, result: R) -> Self;
}

impl Create<Vec<i32>, i64> for Equation {
  fn create(parts: Vec<i32>, result: i64) -> Equation {
    let parts_len = parts.len();
    Equation {
      is_solved: false,
      parts,
      operators: vec![Operator::Add; parts_len.saturating_sub(1)],
      result,
    }
  }
}

impl Equation {
  pub fn evaluate(&self) -> Result<(), CalcError> {
    let mut result = self.parts[0] as i64;
    for (i, op) in self.operators.iter().enumerate() {
      match op {
        Operator::Add => result += self.parts[i + 1] as i64,
        Operator::Multiply => result *= self.parts[i + 1] as i64,
        Operator::Concat => {
          let mut concat_val = self.parts[i + 1] as i64;
          let mut multiplier = 1;
          while concat_val > 0 {
            concat_val /= 10;
            multiplier *= 10;
          }
          result *= multiplier;
          result += self.parts[i + 1] as i64;
        }
      }
    }
    if result == self.result.into() {
      Ok(())
    } else {
      Err(CalcError::InvalidResult(result))
    }
  }

  pub fn is_solvable(&mut self) -> bool {
    self.try_all_combinations(0)
  }

  pub fn try_all_combinations(&mut self, index: usize) -> bool {
    // base case -- the operators are all full
    if index == self.operators.len() {
      match self.evaluate() {
        Ok(_) => {
          self.is_solved = true;
          return true;
        }
        Err(_) => return false,
      }
    }

    self.operators[index] = Operator::Add;
    if self.try_all_combinations(index + 1) {
      return true;
    }

    self.operators[index] = Operator::Multiply;
    if self.try_all_combinations(index + 1) {
      return true;
    }

    self.operators[index] = Operator::Concat;
    if self.try_all_combinations(index + 1) {
      return true;
    }
    false
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_equation() {
        let parts = vec![7, 7, 348, 7, 1, 7, 6, 740, 5, 6, 8];
      let result = 675262124708;
      let eq = Equation::create(parts.clone(), result);

      assert_eq!(eq.is_solved, false);
      assert_eq!(eq.parts, parts);
      assert_eq!(eq.result, result);
      }

      #[test]
      fn test_create_equation_empty_parts() {
        let parts = vec![4];
        let result: i64 = 0;
        let eq = Equation::create(parts.clone(), result);

        assert_eq!(eq.is_solved, false);
        assert_eq!(eq.parts, parts);
        assert_eq!(eq.result, result);
      }

      #[test]
      fn test_check_result() {
          // Example usage
        let parts = vec![11, 6, 16, 20];
        // let operators = vec![Operator::Add, Operator::Add, Operator::Add];
        let result = 292;
        let mut eq = Equation::create(parts.clone(), result);
        eq.is_solvable();
        assert_eq!(eq.is_solved, true);
      }

      #[test]
      fn check_concat_result() {
        let parts = vec![15, 6];
        let result = 156;
        let mut eq = Equation::create(parts.clone(), result);
        eq.is_solvable();
        assert_eq!(eq.is_solved, true);
      }
    }