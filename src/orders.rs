use std::fmt;

use crate::account::Account;

pub type OrderId = u128;
pub type OrderPrice = u128;
pub type OrderQuantity = u128;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrderType {
    Bid,
    Ask
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderType::Bid => write!(f, "BID"),
            OrderType::Ask => write!(f, "ASK")
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Order<'a> {
    id: OrderId,
    owner: &'a mut Account,
    order_type: OrderType,
    price: OrderPrice,
    quantity: OrderQuantity
}

impl<'a> Order<'a> {
    pub fn new(id: OrderId, owner: &'a mut Account, order_type: OrderType,
        price: OrderPrice, quantity: OrderQuantity) -> Self {
        Order {
            id,
            owner,
            order_type,
            price,
            quantity
        }
    }
}

impl fmt::Display for Order<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} {} @ {} for {}", self.id, self.owner.get_name(),
                self.order_type, self.price, self.quantity)
    }
}

