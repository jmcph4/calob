use std::collections::HashMap;

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

