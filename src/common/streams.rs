use ethers::{
    providers::{Middleware, Provider, Ws},
    types::*,
};
use std::sync::Arc;
use tokio::sync::broadcast::Sender;
use tokio_stream::StreamExt;

use crate::common::utils::calculate_next_block_base_fee;

#[derive(Default, Debug, Clone)]
pub struct NewBlock {
    pub block_number: U64,
    pub base_fee: U256,
    pub next_base_fee: U256
}

#[derive(Debug, Clone)]
pub struct NewPendingTx {
    pub added_block
}