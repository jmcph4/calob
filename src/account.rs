use std::collections::HashMap;

pub type AccountId = u128;

#[derive(Clone, Debug, Default)]
pub struct Account {
    id: AccountId,
    name: String,
    balance: f64,
    holdings: HashMap<String, u64>
}

