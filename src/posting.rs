use rust_decimal::Decimal;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::types::AccountKind;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Posting {
    pub account_id: Uuid,
    /// Amount: positive = Debit, negative = Credit
    pub amount: Decimal,
}

impl Posting {
    pub fn debit(account_id: Uuid, amount: Decimal) -> Self {
        Self { account_id, amount: amount.abs() }
    }

    pub fn credit(account_id: Uuid, amount: Decimal) -> Self {
        Self { account_id, amount: -amount.abs() }
    }

    pub fn side(&self) -> AccountKind {
        if self.amount.is_sign_positive() || self.amount.is_zero() {
            AccountKind::Debit
        } else {
            AccountKind::Credit
        }
    }
}
