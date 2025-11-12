use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Posting {
    pub account_id: Uuid,
    pub amount: Decimal, // +debit, -credit
}
