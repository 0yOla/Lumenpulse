//! Standardized error codes for Treasury contract.
//!
//! Error codes are allocated from the shared error registry.
//! Base range: 2000-2999 (ContractId::Treasury = 2)
//! Common errors: 2000 (NotInitialized), 2001 (AlreadyInitialized), 2002 (Unauthorized)

use error_registry::{contract_error_code, ContractId};
use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum TreasuryError {
    /// Contract has not been initialized.
    NotInitialized = contract_error_code!(ContractId::Treasury, 0),
    /// Contract has already been initialized.
    AlreadyInitialized = contract_error_code!(ContractId::Treasury, 1),
    /// Caller is not authorized for this operation.
    Unauthorized = contract_error_code!(ContractId::Treasury, 2),
    /// Invalid amount specified.
    InvalidAmount = contract_error_code!(ContractId::Treasury, 3),
    /// Invalid duration specified.
    InvalidDuration = contract_error_code!(ContractId::Treasury, 4),
    /// Invalid start time specified.
    InvalidStartTime = contract_error_code!(ContractId::Treasury, 5),
    /// Stream not found for beneficiary.
    StreamNotFound = contract_error_code!(ContractId::Treasury, 6),
    /// Nothing to claim at this time.
    NothingToClaim = contract_error_code!(ContractId::Treasury, 7),
    /// Reentrancy detected.
    Reentrancy = contract_error_code!(ContractId::Treasury, 8),
    /// Operation already executed.
    AlreadyExecuted = contract_error_code!(ContractId::Treasury, 9),
    /// Same beneficiary specified for rotation.
    SameBeneficiary = contract_error_code!(ContractId::Treasury, 10),

    // ── Multisig proposal errors ──────────────────────────────
    /// Proposal not found.
    ProposalNotFound = contract_error_code!(ContractId::Treasury, 11),
    /// Proposal not approved.
    ProposalNotApproved = contract_error_code!(ContractId::Treasury, 12),
    /// Proposal already signed.
    ProposalAlreadySigned = contract_error_code!(ContractId::Treasury, 13),
    /// Proposal expired.
    ProposalExpired = contract_error_code!(ContractId::Treasury, 14),
    /// Proposal not active.
    ProposalNotActive = contract_error_code!(ContractId::Treasury, 15),
    /// Wrong proposal action for current state.
    WrongProposalAction = contract_error_code!(ContractId::Treasury, 16),
    /// Invalid multisig configuration.
    InvalidMultisigConfig = contract_error_code!(ContractId::Treasury, 17),
    /// Too many signers for multisig.
    TooManySigners = contract_error_code!(ContractId::Treasury, 18),

    // ── Cliff / schedule preview errors ───────────────────────
    /// Cliff time supplied for a stream was invalid: not yet at start_time,
    /// or cliff_time + step would overflow u64.
    InvalidCliffTime = contract_error_code!(ContractId::Treasury, 19),
    /// A preview query received a zero step or step > max allowed.
    InvalidScheduleStep = contract_error_code!(ContractId::Treasury, 20),
    /// preview_schedule asked for too many entries (caps iteration cost).
    TooManyInstallments = contract_error_code!(ContractId::Treasury, 21),
    /// Total unreleased obligations across all streams exceed held balance.
    Insolvent = contract_error_code!(ContractId::Treasury, 22),
}
