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
    pub ticker: &'a str,
    pub bids: Side<'a>,
    pub asks: Side<'a>,
    pub ltp: OrderPrice
}

