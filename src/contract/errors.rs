use std::fmt;

/// Errors returned by the bounty contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractError {
    /// The bounty was not found.
    BountyNotFound,
    /// The caller is not authorized to perform this action.
    Unauthorized,
    /// The bounty is not open for claims.
    BountyNotOpen,
    /// The contributor's reputation is below the bounty's minimum threshold.
    ReputationTooLow,
    /// The provided description hash is invalid.
    InvalidDescriptionHash,
    /// The reward amount is invalid.
    InvalidReward,
    /// The bounty has already been claimed.
    AlreadyClaimed,
}

impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContractError::BountyNotFound => write!(f, "bounty not found"),
            ContractError::Unauthorized => write!(f, "unauthorized"),
            ContractError::BountyNotOpen => write!(f, "bounty is not open"),
            ContractError::ReputationTooLow => {
                write!(f, "contributor reputation is below the minimum threshold")
            }
            ContractError::InvalidDescriptionHash => write!(f, "invalid description hash"),
            ContractError::InvalidReward => write!(f, "invalid reward"),
            ContractError::AlreadyClaimed => write!(f, "bounty already claimed"),
        }
    }
}

impl std::error::Error for ContractError {}
