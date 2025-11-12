use rust_decimal::Decimal;
use uuid::Uuid;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{error::LedgerError, posting::Posting};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub date: NaiveDate,
    pub description: String,
    pub postings: Vec<Posting>,
    pub is_closing_entry: bool,
    pub is_reversing_entry: bool,
    pub metadata: HashMap<String, String>,
}

impl Transaction {
    pub fn new(id: Uuid, date: NaiveDate, description: impl Into<String>) -> Self {
        Self {
            id,
            date,
            description: description.into(),
            postings: Vec::new(),
            is_closing_entry: false,
            is_reversing_entry: false,
            metadata: HashMap::new(),
        }
    }

    pub fn with_posting(mut self, posting: Posting) -> Self {
        self.postings.push(posting);
        self
    }

    pub fn total(&self) -> Decimal {
        self.postings.iter().map(|p| p.amount).sum()
    }

    pub fn is_balanced(&self) -> bool {
        self.total().is_zero()
    }

    pub fn validate(&self) -> Result<(), LedgerError> {
        if self.postings.is_empty() {
            return Err(LedgerError::EmptyTransaction);
        }
        if !self.is_balanced() {
            return Err(LedgerError::UnbalancedTransaction(self.total()));
        }
        Ok(())
    }
}
