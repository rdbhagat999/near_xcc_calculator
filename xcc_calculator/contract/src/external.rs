use near_sdk::{ext_contract, serde::{Deserialize, Serialize}};

pub const TGAS: u64 = 1_000_000_000_000;
pub const NO_DEPOSIT: u128 = 0;
pub const XCC_SUCCESS: u64 = 1;

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Operation {
  ADD,
  SUB,
  MUL,
  DIV,
  NONE
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct LastOperation {
    pub operator: Operation,
    pub op_1: i32,
    pub op_2: i32,
    pub result: i32,
}
// Interface of this contract, for callbacks
#[ext_contract(this_contract)]
trait Callbacks {
  fn operation_callback(&mut self) -> bool;
  fn last_operation_result_callback(&mut self) -> LastOperation;
}

// Validator interface, for cross-contract calls
#[ext_contract(math_operation)]
trait MathOperation {
  fn execute_operation(&self, op_1: i32, op_2: i32, operator: Operation) -> i32;
  fn last_operation_result(&mut self) -> LastOperation;
}
