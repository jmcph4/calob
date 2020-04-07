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
    pub fn new(id: OrderId, owner: &'a mut Account, r#type: OrderType,
        price: OrderPrice, quantity: OrderQuantity) -> Self {
        Order {
            id,
            owner,
            order_type: r#type,
            price,
            quantity
        }
    }

    pub fn id(&self) -> OrderId {
        self.id
    }

    pub fn owner<'b>(&'b mut self) -> &'b mut Account {
        self.owner
    }

    pub fn r#type(&self) -> OrderType {
        self.order_type
    }

    pub fn price(&self) -> OrderPrice {
        self.price
    }

    pub fn quantity(&self) -> OrderQuantity {
        self.quantity
    }
}

impl fmt::Display for Order<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} {} @ {} for {}", self.id, self.owner.name(),
                self.order_type, self.price, self.quantity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn test_new_normal() {
        let mut actual_acct: Account = Account::new(32, "John Doe".to_string(),
                                                      100000, HashMap::new());

        let mut expected_acct: Account = Account::new(32, "John Doe".to_string(),
                                                        100000, HashMap::new());
        
        let order_id: OrderId = 12;
        let order_owner: &mut Account = &mut actual_acct;
        let order_type: OrderType = OrderType::Bid;
        let order_price: OrderPrice = 330;
        let order_quantity: OrderQuantity = 50;

        let actual_order: Order = Order::new(order_id, order_owner, order_type,
                                                order_price, order_quantity);
        let expected_order: Order = Order {
            id: order_id,
            owner: &mut expected_acct,
            order_type,
            price: order_price,
            quantity: order_quantity
        };

        assert_eq!(actual_order, expected_order);
    }
}

