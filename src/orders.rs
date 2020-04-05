use crate::account::Account;

pub type OrderId = u128;
pub type OrderPrice = u128;
pub type OrderQuantity = u128;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrderType {
    Bid,
    Ask
}

#[derive(Debug, PartialEq, Eq)]
pub struct Order<'a> {
    id: OrderId,
    owner: &'a mut Account,
    order_type: OrderType,
    price: OrderPrice,
    quantity: OrderQuantity
}

