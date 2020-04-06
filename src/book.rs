use std::collections::{BTreeMap, VecDeque};

use crate::order::*;

pub enum BookError {}

type PriceLabel = OrderPrice;
type OrderQueue<'a> = VecDeque<&'a mut Order<'a>>;
pub type Side<'a> = BTreeMap<PriceLabel, OrderQueue<'a>>;

pub type BookId = u128;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Book<'a> {
    id: BookId,
    name: String,
    ticker: String,
    bids: Side<'a>,
    asks: Side<'a>,
    ltp: OrderPrice,
    has_traded: bool
}

impl<'a> Book<'a> {
    pub fn new(id: BookId, name: String, ticker: String) -> Self {
        Book {
            id,
            name,
            ticker,
            bids: Side::new(),
            asks: Side::new(),
            ltp: 0,
            has_traded: false
        }
    }

    pub fn id(&self) -> BookId {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn ticker(&self) -> String {
        self.ticker.clone()
    }

    pub fn ltp(&self) -> Option<OrderPrice> {
        if self.has_traded {
            Some(self.ltp)
        } else {
            None
        }
    }
}

