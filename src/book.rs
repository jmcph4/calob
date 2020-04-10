#![allow(unused_assignments)]
use std::collections::{BTreeMap, VecDeque};

use crate::order::*;

#[derive(Debug)]
pub enum BookError {
    OrderNotFound
}

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
    has_traded: bool,
    order_ids: Vec<OrderId>
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
            has_traded: false,
            order_ids: vec![]
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

    pub fn submit(&mut self, order: &'a mut Order<'a>) ->
        Result<(), BookError> {
        let order_id: OrderId = order.id();
        let price_key: OrderPrice = order.price();
        let order_quantity: OrderQuantity = order.quantity();

        match order.r#type() {
            OrderType::Bid => {
                let mut matched: bool = false;
                
                if !self.order_ids.is_empty() &&
                    price_key >= self.top().unwrap().1 {
                    for curr_queue in self.asks.values_mut() {
                        for _i in 0..curr_queue.len() {
                            let counter_order = curr_queue.pop_front().unwrap();
                            let counter_order_done: bool;
                            let curr_price: OrderPrice = counter_order.price();
                    
                            if curr_price <= price_key {
                                let counter_quantity: OrderPrice =
                                                    counter_order.quantity();
                                if counter_quantity < order_quantity {
                                    Book::payout_order(self.ticker.clone(),
                                        counter_order, Some(curr_price), None)?;
                                    Book::payout_order(self.ticker.clone(),
                                        order, Some(curr_price),
                                        Some(counter_quantity))?;
                                    
                                    /* remove counter order as it is consumed */
                                    counter_order_done = true;
                                    Book::remove_id(&mut self.order_ids,
                                        counter_order.id());
                                } else if counter_quantity == order_quantity {
                                    Book::payout_order(self.ticker.clone(),
                                        counter_order, Some(curr_price), None)?;
                                    Book::payout_order(self.ticker.clone(),
                                        order, Some(curr_price),
                                        Some(counter_quantity))?;
                                    
                                    /* remove counter order as it is consumed */
                                    counter_order_done = true;
                                    Book::remove_id(&mut self.order_ids,
                                        counter_order.id());
                                    
                                    matched = true;
                                    break;
                                } else {
                                    Book::payout_order(self.ticker.clone(),
                                        counter_order, Some(curr_price), None)?;
                                    Book::payout_order(self.ticker.clone(),
                                    order, Some(curr_price),
                                    Some(counter_quantity))?;
                                    
                                    matched = true;
                                    break;
                                }
                                
                                /* restore counter order if not consumed */
                                if !counter_order_done {     
                                    curr_queue.push_back(counter_order);
                                }
                            } else {
                                curr_queue.push_back(counter_order);
                            }
                        }
                    }
                }
                
                if !matched {
                    self.bids.insert(price_key, VecDeque::new());
                    self.bids.get_mut(&price_key).unwrap().push_back(order);
                    self.order_ids.push(order_id);
                } else {
                    self.has_traded = true;
                    self.ltp = price_key;
                }
            },
            OrderType::Ask => {
                let mut matched: bool = false;
                
                if !self.order_ids.is_empty() &&
                    price_key <= self.top().unwrap().0 {
                    for curr_queue in self.bids.values_mut() {
                        for _i in 0..curr_queue.len() {
                            let counter_order = curr_queue.pop_front().unwrap();
                            let counter_order_done: bool;
                            let curr_price: OrderPrice = counter_order.price();
                            
                            if curr_price <= price_key {
                                let counter_quantity: OrderPrice =
                                                    counter_order.quantity();
                            
                                if counter_quantity < order_quantity {
                                    Book::payout_order(self.ticker.clone(),
                                        counter_order, Some(curr_price), None)?;
                                    Book::payout_order(self.ticker.clone(),
                                        order, Some(curr_price),
                                        Some(counter_quantity))?;
                                    
                                    /* remove counter order as it is consumed */
                                    counter_order_done = true;
                                    Book::remove_id(&mut self.order_ids,
                                        counter_order.id());
                                } else if counter_quantity == order_quantity {
                                    Book::payout_order(self.ticker.clone(),
                                        counter_order, Some(curr_price), None)?;
                                    Book::payout_order(self.ticker.clone(),
                                        order, Some(curr_price),
                                        Some(counter_quantity))?;
                                    
                                    /* remove counter order as it is consumed */
                                    counter_order_done = true;
                                    Book::remove_id(&mut self.order_ids,
                                        counter_order.id());
                                    
                                    matched = true;
                                    break;
                                } else {
                                    Book::payout_order(self.ticker.clone(),
                                        counter_order, Some(curr_price), None)?;
                                    Book::payout_order(self.ticker.clone(),
                                        order, Some(curr_price),
                                        Some(counter_quantity))?;
                                    
                                    matched = true;
                                    break;
                                }
                                
                                /* restore counter order if not consumed */
                                if !counter_order_done {     
                                    curr_queue.push_back(counter_order);
                                }
                            } else {
                                curr_queue.push_back(counter_order);
                            }
                        }
                    }
                }
                
                if !matched {
                    self.asks.insert(price_key, VecDeque::new());
                    self.asks.get_mut(&price_key).unwrap().push_back(order);
                    self.order_ids.push(order_id);
                } else {
                    self.has_traded = true;
                    self.ltp = price_key;
                }
            }
        };

        Book::prune_side(&mut self.bids);
        Book::prune_side(&mut self.asks);

        Ok(())
    }

    pub fn cancel(&mut self, id: OrderId) -> Result<(), BookError> {
        if !self.order_ids.contains(&id) {
            return Err(BookError::OrderNotFound);
        }

        let mut index: usize = 0;

        for (_curr_price, curr_queue) in self.bids.iter_mut() {
            for curr_order in curr_queue.iter() {
                if curr_order.id() == id {
                    break;
                }

                index += 1;
            }

            curr_queue.remove(index);
            return Ok(());
        }
        
        Ok(())
    } 
   
    pub fn top(&self) -> Option<(OrderPrice, OrderPrice)> {
        let mut best_bid: OrderPrice = 0;        
        let mut best_ask: OrderPrice = 0;        

        for (price_level, _level_orders) in self.bids.iter() {
            best_bid = *price_level;
            break;
        }

        for (price_level, _level_orders) in self.bids.iter().rev() {
            best_ask = *price_level;
            break;
        }
    
        if best_bid == 0 || best_ask == 0 {
            return None;
        }    

        Some((best_bid, best_ask))
    }
 
    fn payout_order(ticker: String, order: &'a mut Order,
        price: Option<OrderPrice>, quantity: Option<OrderQuantity>) ->
        Result <(), BookError> {
        let actual_price: OrderPrice = match price {
            Some(p) => p,
            None => order.price()
        };

        let actual_quantity: OrderQuantity = match quantity {
            Some(q) => q,
            None => order.quantity()
        };

        let amount: OrderQuantity = actual_price * actual_quantity;

        match order.r#type() {
            OrderType::Bid => {
                order.owner().take_balance(amount);
                order.owner().add_holding(ticker.clone(), actual_quantity);
            },
            OrderType::Ask => {
                order.owner().add_balance(amount);
                order.owner().take_holding(ticker.clone(), actual_quantity);
            }
        };

        Ok(())
    }
    
    fn prune_side(side: &mut Side) {
        let mut prices_to_prune: Vec<OrderPrice> = vec![];
        
        for (price_level, level_orders) in side.iter_mut() {
            if level_orders.is_empty() {
                prices_to_prune.push(*price_level);
            }
        }

        for price in prices_to_prune.iter() {
            side.remove(price);
        }
    }

    fn remove_id(order_ids: &mut Vec<OrderId>, id: OrderId) {
        let mut pos: usize = 0;

        for i in 0..order_ids.len() {
            if order_ids[i] == id {
                pos = i;
                break;
            }
        }

        order_ids.remove(pos);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::iter::FromIterator;
    use crate::account::{Account, AccountHolding};

    #[test]
    fn test_submit_equal_orders() -> Result<(), BookError> {
        let mut holdings: HashMap<String, AccountHolding> = HashMap::new();
        holdings.insert("VOC".to_string(), 20);
        
        let mut actual_account1: Account =
                Account::new(1, "John Doe".to_string(), 2500, HashMap::new());
        let mut actual_account2: Account =
                Account::new(2, "Jane Doe".to_string(), 0, holdings.clone());
        let mut actual_order1: Order = 
                Order::new(1000, &mut actual_account1, OrderType::Bid, 125, 20);
        let mut actual_order2: Order =
                Order::new(1001, &mut actual_account2, OrderType::Ask, 125, 20);
        
        let mut actual_book: Book = Book::new(1,
            "Vereenigde Oostindische Compagnie".to_string(), "VOC".to_string());
        
        actual_book.submit(&mut actual_order1)?;
        actual_book.submit(&mut actual_order2)?;
        
        let expected_book: Book = Book {
            id: 1,
            name: "Vereenigde Oostindische Compagnie".to_string(),
            ticker: "VOC".to_string(),
            bids: Side::new(),
            asks: Side::new(),
            ltp: 125,
            has_traded: true,
            order_ids: vec![]
        };

        let mut expected_holdings2: HashMap<String, AccountHolding> =
            HashMap::new();
        expected_holdings2.insert("VOC".to_string(), 0);
        let mut expected_account1: Account =
                Account::new(1, "John Doe".to_string(), 0, holdings);
        let mut expected_account2: Account =
                Account::new(2, "Jane Doe".to_string(), 2500,
                expected_holdings2);
        let mut expected_order1: Order = 
                Order::new(1000, &mut expected_account1, OrderType::Bid, 125,
                20);
        let mut expected_order2: Order =
                Order::new(1001, &mut expected_account2, OrderType::Ask, 125,
                20);
        
        assert_eq!(actual_book, expected_book);
        assert_eq!(actual_account1, expected_account1);
        assert_eq!(actual_account2, expected_account2);
        
        Ok(())
    }

    #[test]
    fn test_submit_price_mismatch_nocross() -> Result<(), BookError> {
        let mut holdings: HashMap<String, AccountHolding> = HashMap::new();
        holdings.insert("MSFT".to_string(), 20);
        
        let mut actual_account1: Account =
                Account::new(1, "John Doe".to_string(), 2500, HashMap::new());
        let mut actual_account2: Account =
                Account::new(2, "Jane Doe".to_string(), 0, holdings.clone());
        let mut actual_order1: Order = 
                Order::new(1000, &mut actual_account1, OrderType::Bid, 125, 20);
        let mut actual_order2: Order =
                Order::new(1001, &mut actual_account2, OrderType::Ask, 130, 20);
        
        let mut actual_book: Book = Book::new(1,
            "Vereenigde Oostindische Compagnie".to_string(), "VOC".to_string());
        
        actual_book.submit(&mut actual_order1)?;
        actual_book.submit(&mut actual_order2)?;
        
        let mut expected_account1: Account =
                Account::new(1, "John Doe".to_string(), 2500, HashMap::new());
        let mut expected_account2: Account =
                Account::new(2, "Jane Doe".to_string(), 0, holdings.clone());
        let mut expected_order1: Order = 
                Order::new(1000, &mut expected_account1, OrderType::Bid, 125,
                    20);
        let mut expected_order2: Order =
                Order::new(1001, &mut expected_account2, OrderType::Ask, 130,
                    20);
        
        let mut expected_bids: Side = Side::new();
        expected_bids.insert(125, VecDeque::from_iter(
                                    vec![&mut expected_order1]));
        
        let mut expected_asks: Side = Side::new();
        expected_asks.insert(130, VecDeque::from_iter(
                                    vec![&mut expected_order2]));
        
        let expected_book: Book = Book {
            id: 1,
            name: "Vereenigde Oostindische Compagnie".to_string(),
            ticker: "VOC".to_string(),
            bids: expected_bids,
            asks: expected_asks,
            ltp: 0,
            has_traded: false,
            order_ids: vec![1000, 1001]
        };
        
        assert_eq!(actual_book, expected_book);
        assert_eq!(actual_account1, expected_account1);
        assert_eq!(actual_account2, expected_account2);
        
        Ok(())
    }
}

