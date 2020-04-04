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
}

