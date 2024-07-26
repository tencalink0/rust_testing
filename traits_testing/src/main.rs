use crate::blockchain::{Transaction, Block, Blockchain};
use crate::wallets::{Address, Wallet, CryptoName};

mod blockchain;
mod wallets;

use std::time::SystemTime;

fn to_char_arr<const S: usize>(address_raw: String) -> [char; S] {
    let characters: [char; S] = match address_raw.chars().collect::<Vec<_>>().try_into() {
        Ok(characters) => characters,
        Err(_) => panic!("Failed to convert to desired char type")
    };
    characters
}

fn main() {
    const T_SIZES: (usize, usize) = (40, 40);
    const T_SIZE_0: usize = T_SIZES.0;
    const T_SIZE_1: usize = T_SIZES.1;

    let mut bitcoin = Blockchain::<T_SIZE_0, T_SIZE_1> {blocks: Vec::new(), block_size: 64};

    let mut transactions: Vec<Transaction<T_SIZE_0, T_SIZE_1>> = Vec::new();

    let address_raw = String::from("0123456789abcdef0123456789abcdef01234567");
    let address = Address::<T_SIZE_0>(to_char_arr::<T_SIZE_0>(address_raw));

    let address_to_raw = String::from("abcdef0123456789abcdef0123456789abcdef01");
    let address_to = Address::<T_SIZE_1>(to_char_arr::<T_SIZE_1>(address_to_raw));


    let test_transfer = Transaction::new(
        T_SIZES,
        address,
        address_to,
        100,
    );

    transactions.push(test_transfer);
    let block = Block::<T_SIZE_0, T_SIZE_1> {
        prev_block: String::from("0000000000000000"),
        this_block: String::from("0000000000000001"),
        message: None,
        transactions: transactions,
        transaction_sizes: T_SIZES,
        timestamp: Some(SystemTime::now()),
    };

    bitcoin.add_block(block);
    bitcoin.list_blocks();

    println!("{:?}", bitcoin)
}