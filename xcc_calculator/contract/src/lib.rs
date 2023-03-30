use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, PanicOnDefault, AccountId, Gas, Promise, PromiseError, require};

pub mod external;
pub use crate::external::*;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Calculator {
    pub xcc_account: AccountId
}

// Implement the contract structure
#[near_bindgen]
impl Calculator {
    /*
    Only the contract can call this method.
    Initializes the contract.
    @params xcc_account is the address of the other contract.
    */
    #[init]
    #[private]
    pub fn new(xcc_account: AccountId) -> Self {
        log!("Called Calculator::new function");
        assert!(!env::state_exists(), "Already initialized");
        Self {
            xcc_account,
        }
    }

    /*
    Makes a cross-contract call with specified params, returns a promise to a callback.
    Accepts an attched deposit.
    */
    #[payable]
    pub fn perform_operation(&mut self, op_1:i32, op_2:i32, operator: Operation) -> Promise {
        // Create a promise to call MathOperation.execute_operation()

        let deposit_amount = env::attached_deposit();

        require!(deposit_amount >= 1, "Minimum 1 Near required to perform this action.");

        let is_divide_operation = match operator {
            Operation::DIV => true,
            _ => false
        };

        if is_divide_operation {
            // assert_ne!(op_2, 0, "Cannot divide by zero");
            require!(op_2 > 0, "op_2 must be greater than 0");
        }

        let promise = math_operation::ext(self.xcc_account.clone())
        .with_static_gas(Gas(9*TGAS))
        .with_attached_deposit(deposit_amount)
        .execute_operation(op_1, op_2, operator);

        return promise.then( // Create a promise to callback operation_callback
            Self::ext(env::current_account_id())
                .with_static_gas(Gas(9*TGAS))
                .operation_callback()
        )
    }

    /*
    Callback for perform_operation method.
    returns boolean value depending on call_result param.
    */
    #[private]
    pub fn operation_callback(&mut self, #[callback_result] call_result: Result<i32, PromiseError>) -> bool {
        // Check if the promise succeeded by calling the method outlined in external.rs
        if call_result.is_err() {
            env::log_str("perform_operation failed...");
            return false;
        } else {
            env::log_str("perform_operation was successful!");
            return true;
        }
    }

    /*
    Makes a cross-contract call, returns a promise to a callback.
    */
    pub fn query_last_operation_result(&self) -> Promise {
        // Create a promise to call MathOperation.last_operation_result()
        let promise = math_operation::ext(self.xcc_account.clone())
        .with_static_gas(Gas(9*TGAS))
        .last_operation_result();

        return promise.then( // Create a promise to callback last_operation_result_callback
            Self::ext(env::current_account_id())
                .with_static_gas(Gas(9*TGAS))
                .last_operation_result_callback()
        )
    }

    /*
    Callback for perform_operation method.
    Can only be called by contract.
    returns boolean value depending on call_result param.
    */
    #[private]
    pub fn last_operation_result_callback(&mut self, #[callback_result] call_result: Result<LastOperation, PromiseError>) -> LastOperation {
        // Check if the promise succeeded by calling the method outlined in external.rs
        if call_result.is_err() {
            log!("There was an error contacting MathOperation Contract");
            return LastOperation {
                    operator: Operation::NONE,
                    op_1: 0,
                    op_2: 0,
                    result: 0,
                }
        }

        log!("{:?}", call_result);

        // Return the result
        let result: LastOperation = call_result.unwrap();
        return result
    }
}


/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
}
