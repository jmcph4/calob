use std::collections::HashMap;
use std::fmt;

use thiserror::Error;

#[derive(Copy, Clone, Debug, Error)]
pub enum AccountError {
    InsufficientFunds,
    InsufficientHoldings,
    BalanceOutOfBounds,
    HoldingOutOfBounds,
    AssetNotFound
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccountError::InsufficientFunds => write!(f, "Insufficient Funds"),
            AccountError::InsufficientHoldings =>
                write!(f, "Insufficient Holdings"),
            AccountError::BalanceOutOfBounds =>
                write!(f, "Balance (or difference in) too large or too small"),
            AccountError::HoldingOutOfBounds =>
                write!(f, "Holding (or difference in) too large or too small"),
            AccountError::AssetNotFound =>
                write!(f, "No such asset in portfolio")
        }
    }
}

pub type AccountId = u128;
pub type AccountBalance = u128;
pub type AccountHolding = u128;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Account {
    id: AccountId,
    name: String,
    balance: AccountBalance,
    holdings: HashMap<String, AccountHolding>
}

impl Account {
    pub fn new(id: AccountId, name: String, balance: AccountBalance,
        holdings: HashMap<String, AccountHolding>) -> Self {
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

    pub fn holding(&self, ticker: String) -> Option<AccountHolding> {
        if !self.holdings.contains_key(&ticker) {
            return None;
        }

        Some(self.holdings[&ticker])
    }

    pub fn set_holding(&mut self, ticker: String, quantity: AccountHolding) {
        self.holdings.entry(ticker).or_insert(quantity);
    }

    pub fn add_balance(&mut self, amount: AccountBalance) ->
        Result<(), AccountError> {
        /* bounds check */
        if self.balance.checked_add(amount).is_none() {
            return Err(AccountError::BalanceOutOfBounds);
        }
        
        self.balance += amount;
        Ok(())
    }

    pub fn take_balance(&mut self, amount: AccountBalance) ->
        Result<(), AccountError> {
        /* bounds check */
        if amount > self.balance {
            return Err(AccountError::BalanceOutOfBounds);
        }

        self.balance -= amount;
        Ok(())
    }

    #[allow(clippy::map_entry)]
    pub fn add_holding(&mut self, ticker: String, amount: AccountHolding) ->
        Result<(), AccountError> {
        if !self.holdings.contains_key(&ticker) {
            self.holdings.insert(ticker, amount);
        } else {
            /* bounds check */
            if self.holdings.get_mut(&ticker).unwrap().checked_add(amount).
                is_none() {
                return Err(AccountError::HoldingOutOfBounds);
            }

            *self.holdings.get_mut(&ticker).unwrap() += amount;
        }

        Ok(())
    }

    pub fn take_holding(&mut self, ticker: String, amount: AccountHolding) ->
        Result<(), AccountError> {
        /* bounds check */
        if !self.holdings.contains_key(&ticker) {
            return Err(AccountError::AssetNotFound);
        }

        if amount > self.holdings[&ticker] {
            return Err(AccountError::HoldingOutOfBounds);
        }

        *self.holdings.get_mut(&ticker).unwrap() -= amount;
        Ok(())
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
        let account_holdings: HashMap<String, AccountHolding> = HashMap::new();
        
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

