use uuid::Uuid;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: Uuid,
    pub date: NaiveDate,
    pub description: String,
}
