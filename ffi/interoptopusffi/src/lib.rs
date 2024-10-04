extern crate core;

pub mod exports;

use interoptopus::{function, Inventory, InventoryBuilder};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Vector a(len{a}) is bigger than b(len({b}), can't add a to b")]
    SizeError {a: u64, b: u64}
}

type Result<T, E=ApiError> = std::result::Result<T, E>;

pub fn ffi_inventory() -> Inventory {
    {
        InventoryBuilder::new().register(function!(exports::add_a_to_b_copy)).register(function!(exports::gen_random_copy)).inventory()
    }
}
