use serde::{Deserialize, Serialize};

/// The five core account types in double-entry accounting.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

impl AccountType {
    /// Returns the natural balance side (Debit/Credit).
    pub fn natural_balance(&self) -> AccountKind {
        match self {
            AccountType::Asset | AccountType::Expense => AccountKind::Debit,
            _ => AccountKind::Credit,
        }
    }

    /// Returns `true` for temporary accounts (Revenue/Expense).
    pub fn is_temporary(&self) -> bool {
        matches!(self, AccountType::Revenue | AccountType::Expense)
    }
}

/// The two sides of an account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountKind {
    Debit,
    Credit,
}

impl std::fmt::Display for AccountKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            AccountKind::Debit => "Debit",
            AccountKind::Credit => "Credit",
        })
    }
}
