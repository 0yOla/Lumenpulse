//! Standardized error codes for VestingWallet contract.
//!
//! Error codes are allocated from the shared error registry.
//! Base range: 3000-3999 (ContractId::VestingWallet = 3)
//! Common errors: 3000 (NotInitialized), 3001 (AlreadyInitialized), 3002 (Unauthorized)

use soroban_sdk::contracterror;
use error_registry::{contract_error_code, ContractId};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VestingError {
    /// Contract has not been initialized.
    NotInitialized = contract_error_code!(ContractId::VestingWallet, 0),
    /// Contract has already been initialized.
    AlreadyInitialized = contract_error_code!(ContractId::VestingWallet, 1),
    /// Caller is not authorized for this operation.
    Unauthorized = contract_error_code!(ContractId::VestingWallet, 2),
    /// Vesting schedule not found for beneficiary.
    VestingNotFound = contract_error_code!(ContractId::VestingWallet, 3),
    /// Invalid amount specified.
    InvalidAmount = contract_error_code!(ContractId::VestingWallet, 4),
    /// Invalid duration specified.
    InvalidDuration = contract_error_code!(ContractId::VestingWallet, 5),
    /// Invalid start time specified.
    InvalidStartTime = contract_error_code!(ContractId::VestingWallet, 6),
    /// Nothing to claim at this time.
    NothingToClaim = contract_error_code!(ContractId::VestingWallet, 7),
    /// Insufficient balance for operation.
    InsufficientBalance = contract_error_code!(ContractId::VestingWallet, 8),
    /// Reentrancy detected.
    Reentrancy = contract_error_code!(ContractId::VestingWallet, 9),
    /// Delegate not authorized for this operation.
    DelegateNotAuthorized = contract_error_code!(ContractId::VestingWallet, 10),
}