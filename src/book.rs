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

    pub fn submit(&mut self, order: &'a mut Order<'a>) ->
        Result<(), BookError> {
        let price_key: OrderPrice = order.price();
        let order_quantity: OrderQuantity = order.quantity();

        match order.r#type() {
            OrderType::Bid => {
                let mut matched: bool = false;
                
                for curr_queue in self.asks.values_mut() {
                    /*let mut curr_queue: &mut VecDeque<&'a mut Order> =
                        self.asks.get_mut(curr_price).unwrap();*/
                    
                    for _i in 0..curr_queue.len() {
                        let counter_order = curr_queue.pop_front().unwrap();
                        let mut counter_order_done: bool = false;
                        let curr_price: OrderPrice = counter_order.price();
                        
                        if curr_price <= price_key {
                            let counter_quantity: OrderPrice =
                                                    counter_order.quantity();
                            
                            if counter_quantity < order_quantity {
                                counter_order.owner().add_balance(curr_price *
                                                            counter_quantity);
                                counter_order.owner().take_holding(
                                                            self.ticker.clone(),
                                                            counter_quantity);

                                order.owner().take_balance(curr_price *
                                                            counter_quantity);
                                order.owner().add_holding(self.ticker.clone(),
                                                            counter_quantity);

                                /* remove counter order as it is consumed */
                                counter_order_done = true;
                            } else if counter_quantity == order_quantity {
                                counter_order.owner().add_balance(curr_price *
                                                            counter_quantity);
                                counter_order.owner().take_holding(
                                                            self.ticker.clone(),
                                                            counter_quantity);

                                order.owner().take_balance(curr_price *
                                                            counter_quantity);
                                order.owner().add_holding(self.ticker.clone(),
                                                            counter_quantity);
                                
                                /* remove counter order as it is consumed */
                                counter_order_done = true;
 
                                matched = true;
                                break;
                            } else {
                                counter_order.owner().take_balance(curr_price *
                                                            order_quantity);
                                counter_order.owner().add_holding(
                                                            self.ticker.clone(),
                                                            order_quantity);

                                order.owner().add_balance(curr_price *
                                                            order_quantity);
                                order.owner().take_holding(self.ticker.clone(),
                                                            order_quantity);
                                
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

                if !matched {
                    self.bids.insert(price_key, VecDeque::new());
                    self.bids.get_mut(&price_key).unwrap().push_back(order);
                } else {
                    self.has_traded = true;
                    self.ltp = price_key;
                }
            },
            OrderType::Ask => {
                let mut matched: bool = false;
                
                for curr_queue in self.bids.values_mut() {
                    /*let mut curr_queue: &mut VecDeque<&'a mut Order> =
                        self.asks.get_mut(curr_price).unwrap();*/
                    
                    for _i in 0..curr_queue.len() {
                        let counter_order = curr_queue.pop_front().unwrap();
                        let mut counter_order_done: bool = false;
                        let curr_price: OrderPrice = counter_order.price();
                        
                        if curr_price <= price_key {
                            let counter_quantity: OrderPrice =
                                                    counter_order.quantity();
                            
                            if counter_quantity < order_quantity {
                                counter_order.owner().take_balance(curr_price *
                                                            counter_quantity);
                                counter_order.owner().add_holding(
                                                            self.ticker.clone(),
                                                            counter_quantity);

                                order.owner().add_balance(curr_price *
                                                            counter_quantity);
                                order.owner().take_holding(self.ticker.clone(),
                                                            counter_quantity);

                                /* remove counter order as it is consumed */
                                counter_order_done = true;
                            } else if counter_quantity == order_quantity {
                                counter_order.owner().take_balance(curr_price *
                                                            counter_quantity);
                                counter_order.owner().add_holding(
                                                            self.ticker.clone(),
                                                            counter_quantity);

                                order.owner().add_balance(curr_price *
                                                            counter_quantity);
                                order.owner().take_holding(self.ticker.clone(),
                                                            counter_quantity);
                                
                                /* remove counter order as it is consumed */
                                counter_order_done = true;
 
                                matched = true;
                                break;
                            } else {
                                counter_order.owner().add_balance(curr_price *
                                                            order_quantity);
                                counter_order.owner().take_holding(
                                                            self.ticker.clone(),
                                                            order_quantity);

                                order.owner().take_balance(curr_price *
                                                            order_quantity);
                                order.owner().add_holding(self.ticker.clone(),
                                                            order_quantity);
                                
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

                if !matched {
                    self.asks.insert(price_key, VecDeque::new());
                    self.asks.get_mut(&price_key).unwrap().push_back(order);
                } else {
                    self.has_traded = true;
                    self.ltp = price_key;
                }
            }
        };

        Ok(())
    } 
}

