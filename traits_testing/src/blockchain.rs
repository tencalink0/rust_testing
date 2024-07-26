use std::time::SystemTime;
use rand::prelude::*;

use crate::wallets::{Address, Wallet, CryptoName};

#[derive(Debug)]
pub struct Block<const S1: usize, const S2: usize> {
    pub prev_block: String,
    pub this_block: String,
    pub message: Option<String>,
    pub transactions: Vec<Transaction<S1, S2>>,
    pub transaction_sizes: (usize, usize),
    pub timestamp: Option<SystemTime>,
}

#[derive(Debug)]
pub struct Transaction<const S1: usize, const S2: usize> {
    pub sizes: (usize, usize),
    pub adr_from: Address<S1>,
    pub adr_to: Address<S2>,
    pub amount: usize,
}

impl<const S1: usize, const S2: usize> Transaction<S1, S2> {
    pub fn new(sizes: (usize, usize), adr_from: Address<S1>, adr_to: Address<S2>, amount: usize) -> Self {
        Transaction {
            sizes,
            adr_from,
            adr_to,
            amount,
        }
    }
}

#[derive(Debug)]
pub struct Blockchain<const S1: usize, const S2: usize> {
    pub blocks: Vec<Block<S1, S2>>, 
    pub block_size: usize,
}

impl<const S1: usize, const S2:usize> Blockchain<S1, S2> {
    pub fn add_block(&mut self, block: Block<S1, S2>) {
        self.blocks.push(block);
    }
    pub fn list_blocks(&self) {
        let block_count = self.blocks.len() - 1;
        for n in 0..=block_count {
            println!{"Block {}: {} \n", block_count, self.blocks[n].this_block}
        }
    }
}