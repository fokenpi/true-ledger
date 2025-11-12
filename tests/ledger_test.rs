use true_ledger::*;
use rust_decimal_macros::dec;
use uuid::Uuid;
use chrono::NaiveDate;

// Import AccountType explicitly (needed in tests)
use true_ledger::types::AccountType;

#[test]
fn test_basic_double_entry() {
    let mut engine = LedgerEngine::new();

    let cash_id = Uuid::new_v4();
    let revenue_id = Uuid::new_v4();

    engine.add_account(Account::new(cash_id, "Cash".into(), AccountType::Asset));
    engine.add_account(Account::new(revenue_id, "Sales".into(), AccountType::Revenue));

    let tx = Transaction::new(
        Uuid::new_v4(),
        NaiveDate::from_ymd_opt(2025, 11, 13).unwrap(),
        "Sale for $100"
    )
    .with_posting(Posting::debit(cash_id, dec!(100)))
    .with_posting(Posting::credit(revenue_id, dec!(100)));

    assert!(engine.record_transaction(tx).is_ok());

    assert_eq!(engine.get_account_balance(&cash_id), dec!(100));
    assert_eq!(engine.get_account_balance(&revenue_id), dec!(-100));

    let tb = engine.generate_trial_balance();
    assert!(tb.is_balanced);
    assert_eq!(tb.total_debits, dec!(100));
    assert_eq!(tb.total_credits, dec!(100));
}
