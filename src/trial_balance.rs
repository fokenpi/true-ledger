use rust_decimal::Decimal;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::types::{AccountType, AccountKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalance {
    pub account_id: Uuid,
    pub account_name: String,
    pub account_type: AccountType,
    pub debit: Decimal,
    pub credit: Decimal,
    pub net: Decimal,
    pub is_abnormal: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialBalance {
    pub accounts: Vec<AccountBalance>,
    pub total_debits: Decimal,
    pub total_credits: Decimal,
    pub is_balanced: bool,
}
