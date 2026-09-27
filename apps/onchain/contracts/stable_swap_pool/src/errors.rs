//! Standardized error codes for StableSwapPool contract.
//!
//! Error codes are allocated from the shared error registry.
//! Base range: 6000-6999 (ContractId::StableSwapPool = 6)
//! Common errors: 6000 (NotInitialized), 6001 (AlreadyInitialized), 6002 (Unauthorized)

use error_registry::{contract_error_code, ContractId};
use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum StableSwapError {
    /// Contract has not been initialized.
    NotInitialized = contract_error_code!(ContractId::StableSwapPool, 0),
    /// Contract has already been initialized.
    AlreadyInitialized = contract_error_code!(ContractId::StableSwapPool, 1),
    /// Caller is not authorized for this operation.
    Unauthorized = contract_error_code!(ContractId::StableSwapPool, 2),
    /// Invalid amount (zero or negative).
    InvalidAmount = contract_error_code!(ContractId::StableSwapPool, 3),
    /// Slippage tolerance exceeded.
    SlippageExceeded = contract_error_code!(ContractId::StableSwapPool, 4),
    /// Insufficient LP balance for removal.
    InsufficientBalance = contract_error_code!(ContractId::StableSwapPool, 5),
    /// Reentrancy detected.
    Reentrancy = contract_error_code!(ContractId::StableSwapPool, 6),
    /// Pool reserves would underflow.
    Underflow = contract_error_code!(ContractId::StableSwapPool, 7),
    /// Math overflow in calculation.
    Overflow = contract_error_code!(ContractId::StableSwapPool, 8),
    /// Token transfer failed.
    TransferFailed = contract_error_code!(ContractId::StableSwapPool, 9),
}
