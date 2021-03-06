use lazy_static::lazy_static;
use crate::{APP_CONFIG, LocalSummary};
use std::sync::Mutex;

lazy_static!(
    static ref LATEST_ORDERBOOK: Mutex<LocalSummary> = initialize_store();
);

fn initialize_store() -> Mutex<LocalSummary> {
    // dbg!("The orderbook is being initialized.");
    let initial_orderbook: LocalSummary = LocalSummary::new(&APP_CONFIG.currency_pair.clone());
    Mutex::new(initial_orderbook)
}

pub fn update_merged_orderbook(new_orderbook: LocalSummary) {
    *LATEST_ORDERBOOK.lock().unwrap() = new_orderbook;
    // println!("{:?}", LATEST_ORDERBOOK.lock().as_ref());
    // dbg!("The orderbook was updated!");
}

pub fn get_merged_orderbook() -> LocalSummary {
    // dbg!("The orderbook is being accessed.");
    let original = LATEST_ORDERBOOK.lock().unwrap();
    original.clone()
}