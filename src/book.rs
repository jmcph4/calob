use std::collections::{BTreeMap, VecDeque};

use crate::order::*;

pub enum BookError {}

type PriceLabel = OrderPrice;
type OrderQueue<'a> = VecDeque<&'a mut Order<'a>>;
pub type Side<'a> = BTreeMap<PriceLabel, OrderQueue<'a>>;

pub type BookId = u128;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Book<'a> {
    pub id: BookId,
    pub name: String,
    pub ticker: String,
    pub bids: Side<'a>,
    pub asks: Side<'a>,
    pub ltp: OrderPrice
}

impl<'a> Book<'a> {
    pub fn new(id: BookId, name: String, ticker: String) -> Self {
        Book {
            id,
            name,
            ticker,
            bids: Side::new(),
            asks: Side::new(),
            ltp: 0
        }
    }
}

