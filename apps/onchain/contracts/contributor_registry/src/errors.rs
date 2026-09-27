//! Standardized error codes for ContributorRegistry contract.
//!
//! Error codes are allocated from the shared error registry.
//! Base range: 1000-1999 (ContractId::ContributorRegistry = 1)
//! Common errors: 1000 (NotInitialized), 1001 (AlreadyInitialized), 1002 (Unauthorized)

use error_registry::{contract_error_code, ContractId};
use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContributorError {
    /// Contract has not been initialized.
    NotInitialized = contract_error_code!(ContractId::ContributorRegistry, 0),
    /// Contract has already been initialized.
    AlreadyInitialized = contract_error_code!(ContractId::ContributorRegistry, 1),
    /// Caller is not authorized for this operation.
    Unauthorized = contract_error_code!(ContractId::ContributorRegistry, 2),
    /// Contributor not found in registry.
    ContributorNotFound = contract_error_code!(ContractId::ContributorRegistry, 3),
    /// Contributor already exists in registry.
    ContributorAlreadyExists = contract_error_code!(ContractId::ContributorRegistry, 4),
    /// Invalid GitHub handle format.
    InvalidGitHubHandle = contract_error_code!(ContractId::ContributorRegistry, 5),
    /// Reputation score would overflow.
    ReputationOverflow = contract_error_code!(ContractId::ContributorRegistry, 6),
    /// GitHub handle is already taken by another contributor.
    GitHubHandleTaken = contract_error_code!(ContractId::ContributorRegistry, 7),
    /// Invalid multisig configuration.
    InvalidMultisigConfig = contract_error_code!(ContractId::ContributorRegistry, 8),
    /// Too many signers for multisig.
    TooManySigners = contract_error_code!(ContractId::ContributorRegistry, 9),
    /// Proposal not found.
    ProposalNotFound = contract_error_code!(ContractId::ContributorRegistry, 10),
    /// Invalid proposal status for this operation.
    InvalidProposalStatus = contract_error_code!(ContractId::ContributorRegistry, 11),
    /// Proposal has expired.
    ProposalExpired = contract_error_code!(ContractId::ContributorRegistry, 12),
    /// Already signed this proposal.
    AlreadySigned = contract_error_code!(ContractId::ContributorRegistry, 13),
    /// Below multisig threshold.
    BelowThreshold = contract_error_code!(ContractId::ContributorRegistry, 14),
    /// Invalid nonce for gasless registration.
    InvalidNonce = contract_error_code!(ContractId::ContributorRegistry, 15),
    /// Invalid signature for gasless registration.
    InvalidSignature = contract_error_code!(ContractId::ContributorRegistry, 16),
    /// Attestation is not active.
    AttestationNotActive = contract_error_code!(ContractId::ContributorRegistry, 17),
    /// Attestation is not suspended.
    AttestationNotSuspended = contract_error_code!(ContractId::ContributorRegistry, 18),
    /// Attestation already revoked.
    AttestationAlreadyRevoked = contract_error_code!(ContractId::ContributorRegistry, 19),
    /// The Contribution scope (register_contributor, gasless_register) is paused.
    ContributionScopePaused = contract_error_code!(ContractId::ContributorRegistry, 20),
    /// The Governance scope (multisig proposals and admin-gated mutations) is paused.
    GovernanceScopePaused = contract_error_code!(ContractId::ContributorRegistry, 21),
}
