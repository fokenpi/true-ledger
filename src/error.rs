use rust_decimal::Decimal;
use thiserror::Error;
use uuid::Uuid;

use crate::types::AccountKind;

#[derive(Error, Debug, serde::Serialize, serde::Deserialize)]
pub enum LedgerError {
    #[error("Unbalanced transaction (sum = {0})")]
    UnbalancedTransaction(Decimal),
    #[error("Account not found: {0}")]
    AccountNotFound(Uuid),
    #[error("Transaction contains no postings")]
    EmptyTransaction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Strictness {
    #[default]
    Lenient,
    Strict,
}
