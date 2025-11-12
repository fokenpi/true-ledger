use std::collections::HashMap;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;
use chrono::NaiveDate;

use crate::{
    error::{LedgerError, Strictness},
    types::{AccountType, AccountKind},
    account::Account,
    posting::Posting,
    transaction::Transaction,
    trial_balance::{AccountBalance, TrialBalance},
};

#[derive(Debug, Clone)]
pub struct LedgerEngine {
    chart_of_accounts: HashMap<Uuid, Account>,
    account_balances: HashMap<Uuid, Decimal>,
    general_ledger: Vec<Transaction>,
    strictness: Strictness,
}

impl LedgerEngine {
    pub fn new() -> Self {
        Self {
            chart_of_accounts: HashMap::new(),
            account_balances: HashMap::new(),
            general_ledger: Vec::new(),
            strictness: Strictness::default(),
        }
    }

    pub fn add_account(&mut self, account: Account) {
        let id = account.id;
        self.chart_of_accounts.insert(id, account);
        self.account_balances.insert(id, dec!(0));
    }

    pub fn record_transaction(&mut self, tx: Transaction) -> Result<(), LedgerError> {
        tx.validate()?;

        // Validate accounts exist
        for p in &tx.postings {
            if !self.chart_of_accounts.contains_key(&p.account_id) {
                return Err(LedgerError::AccountNotFound(p.account_id));
            }
        }

        // Update balances
        for p in &tx.postings {
            *self.account_balances.get_mut(&p.account_id).unwrap() += p.amount;
        }

        self.general_ledger.push(tx);
        Ok(())
    }

    pub fn get_account_balance(&self, account_id: &Uuid) -> Decimal {
        *self.account_balances.get(account_id).unwrap_or(&dec!(0))
    }

    pub fn generate_trial_balance(&self) -> TrialBalance {
        let mut accounts = Vec::new();
        let mut total_debits = dec!(0);
        let mut total_credits = dec!(0);

        for (id, account) in &self.chart_of_accounts {
            let net = self.get_account_balance(id);
            let (debit, credit) = if net >= dec!(0) { (net, dec!(0)) } else { (dec!(0), -net) };
            let expected = account.natural_balance();
            let actual = if net.is_sign_positive() || net.is_zero() { AccountKind::Debit } else { AccountKind::Credit };
            let is_abnormal = net != dec!(0) && expected != actual;

            accounts.push(AccountBalance {
                account_id: *id,
                account_name: account.name.clone(),
                account_type: account.account_type.clone(),
                debit,
                credit,
                net,
                is_abnormal,
            });

            total_debits += debit;
            total_credits += credit;
        }

        accounts.sort_by_key(|ab| ab.account_id);

        TrialBalance {
            accounts,
            total_debits,
            total_credits,
            is_balanced: total_debits == total_credits,
        }
    }
}
