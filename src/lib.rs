//! true-ledger: A double-entry ledger engine in Rust.
pub mod types;
pub mod error;

pub mod account;
pub mod trial_balance;
pub mod posting;
pub mod transaction;
pub mod engine;

// Re-exports for convenience
pub use account::Account;
pub use posting::Posting;
pub use transaction::Transaction;
pub use trial_balance::{AccountBalance, TrialBalance};
pub use engine::LedgerEngine;
