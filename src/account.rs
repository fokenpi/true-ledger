use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::types::{AccountType, AccountKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub account_type: AccountType,
    pub parent_id: Option<Uuid>,
}

impl Account {
    pub fn new(id: Uuid, name: String, account_type: AccountType) -> Self {
        Self {
            id,
            name,
            account_type,
            parent_id: None,
        }
    }

    pub fn with_parent(mut self, parent_id: Uuid) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    pub fn natural_balance(&self) -> AccountKind {
        self.account_type.natural_balance()
    }
}
