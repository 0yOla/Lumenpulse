//! Standardized error codes for PricingAdapter contract.
//!
//! Error codes are allocated from the shared error registry.
//! Base range: 4000-4999 (ContractId::PricingAdapter = 4)
//! Common errors: 4000 (NotInitialized), 4001 (AlreadyInitialized), 4002 (Unauthorized)

use soroban_sdk::contracterror;
use error_registry::{contract_error_code, ContractId};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PricingAdapterError {
    /// Contract has not been initialized.
    NotInitialized = contract_error_code!(ContractId::PricingAdapter, 0),
    /// Contract has already been initialized.
    AlreadyInitialized = contract_error_code!(ContractId::PricingAdapter, 1),
    /// Caller is not authorized for this operation.
    Unauthorized = contract_error_code!(ContractId::PricingAdapter, 2),
    /// Price not found for asset.
    PriceNotFound = contract_error_code!(ContractId::PricingAdapter, 3),
    /// Invalid price value.
    InvalidPrice = contract_error_code!(ContractId::PricingAdapter, 4),
    /// Price is stale (too old).
    StalePrice = contract_error_code!(ContractId::PricingAdapter, 5),
    /// Price has been invalidated.
    PriceInvalidated = contract_error_code!(ContractId::PricingAdapter, 6),
}