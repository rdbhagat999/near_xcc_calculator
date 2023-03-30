use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{log, near_bindgen, PanicOnDefault, AccountId, env, Promise};

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Operation {
  ADD,
  SUB,
  MUL,
  DIV,
  NONE
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct LastOperation {
    operator: Operation,
    op_1: i32,
    op_2: i32,
    result: i32,
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MathOperation {
    operation_history: LookupMap<AccountId, LastOperation>,
}

// Implement the contract structure
#[near_bindgen]
impl MathOperation {

     /*
    Only the contract can call this method.
    Initializes the contract.
    */
    #[init]
    #[private]
    pub fn new() -> Self {
        log!("Called MathOperation::new function");
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "Only the contract may call this method"
        );
        Self {
            operation_history: LookupMap::new(b"h"),
        }
    }

    /*
    Accepts an attched deposit.
    Saves operation result mapped to signer accountId.
    Transfers surplus to signer_id.
    returns an i32 after performing specified operation on the given operands.
    */
    #[payable]
    pub fn execute_operation(&mut self, op_1: i32, op_2: i32, operator: Operation) -> i32 {
        let storage_used_before = env::storage_usage();
        log!("storage_used_before: {} bytes", storage_used_before);

        let signer_account_id = env::signer_account_id();
        log!("signer_account_id: {}", signer_account_id);

        let predecessor_account_id = env::predecessor_account_id();
        log!("predecessor_account_id: {}", predecessor_account_id);

        let deposit_amount = env::attached_deposit();
        log!("deposit_amount: {} yN", deposit_amount);

        let result = match operator {
            Operation::ADD => i32::checked_add(op_1, op_2).unwrap_or(0),
            Operation::SUB => i32::checked_sub(op_1, op_2).unwrap_or(0),
            Operation::MUL => i32::checked_mul(op_1, op_2).unwrap_or(0),
            Operation::DIV => i32::checked_div(op_1, op_2).unwrap_or(0),
            Operation::NONE => 0i32,
        };

        let last_operation = LastOperation {
            operator,
            op_1,
            op_2,
            result
        };

        self.operation_history.insert(&signer_account_id, &last_operation);

        let storage_cost_per_byte = env::storage_byte_cost();
        log!("storage_cost_per_byte: {} yN", storage_cost_per_byte);

        let storage_used_after = env::storage_usage();
        log!("storage_used_after: {} bytes", storage_used_after);

        let payable_storage = storage_used_after - storage_used_before;
        log!("payable_storage: {} bytes", payable_storage);

        let final_storage_cost = u128::checked_mul(storage_cost_per_byte, payable_storage.into()).unwrap();
        log!("final storage cost: {} yN", final_storage_cost);

        let surplus = u128::checked_sub(deposit_amount, final_storage_cost).unwrap();

        log!("surplus: {}", surplus);

        if &surplus > &0 {
            Promise::new(signer_account_id.clone()).transfer(surplus.clone());
            log!("Transferred surplus: {} yN to account_id: {}", &surplus, &signer_account_id);
        }

        return result;

    }

    /*
    Fetches the result of the last operation performed by the signer account.
    */
    pub fn last_operation_result(&mut self) -> LastOperation {

        let default_last_operation = LastOperation {
            operator: Operation::NONE,
            op_1: 0,
            op_2: 0,
            result: 0
        };

        let signer_id = env::signer_account_id();
        let last_result = self.operation_history.get(&signer_id.into()).unwrap_or_else(|| default_last_operation);
        return last_result;
    }

}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {}
