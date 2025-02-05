use crate::guard::Vector;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum MoveError {
  OutOfBounds,
  StuckInLoop(Vector),
  UnknownChar(char),
}