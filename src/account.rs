use std::collections::HashMap;
use std::fmt;

use thiserror::Error;

#[derive(Copy, Clone, Debug, Error)]
pub enum AccountError {
    InsufficientFunds,
    InsufficientHoldings
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccountError::InsufficientFunds => write!(f, "Insufficient Funds"),
            AccountError::InsufficientHoldings =>
                write!(f, "Insufficient Holdings")
        }
    }
}

pub type AccountId = u128;
pub type AccountBalance = u128;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Account {
    id: AccountId,
    name: String,
    balance: AccountBalance,
    holdings: HashMap<String, u64>
}

impl Account {
    pub fn new(id: AccountId, name: String, balance: AccountBalance,
        holdings: HashMap<String, u64>) -> Self {
        Account {
            id,
            name,
            balance,
            holdings
        }
    }    

    pub fn id(&self) -> AccountId {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn balance(&self) -> AccountBalance {
        self.balance
    }

    pub fn set_balance(&mut self, balance: AccountBalance) {
        self.balance = balance;
    }

    pub fn holding(&self, ticker: String) -> Option<u64> {
        if !self.holdings.contains_key(&ticker) {
            return None;
        }

        Some(self.holdings[&ticker])
    }

    pub fn set_holding(&mut self, ticker: String, quantity: u64) {
        self.holdings.insert(ticker, quantity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_normal() {
        let account_id: AccountId = 12;
        let account_name: String = "John Doe".to_string();
        let account_balance: AccountBalance = 33000;
        let account_holdings: HashMap<String, u64> = HashMap::new();
        
        let expected_account: Account = Account {
            id: account_id,
            name: account_name.clone(),
            balance: account_balance,
            holdings: account_holdings.clone()
        };
        
        let actual_account: Account = Account::new(account_id, account_name,
                                        account_balance, account_holdings);
        
        assert_eq!(actual_account, expected_account);
    }
}

