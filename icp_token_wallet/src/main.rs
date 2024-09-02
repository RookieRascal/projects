fuse ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::*;
use std::collections::HashMap;

type Token = u64;

#[derive(Default)]
struct Wallet {
    balance: Token,
    ledger: HashMap<String, Token>,
}

#[init]
fn init() -> Wallet {
    Wallet::default()
}

#[update]
fn send(to: String, amount: Token) -> Result<(), String> {
    let wallet = ic_cdk::storage::get_mut::<Wallet>();

    if wallet.balance < amount {
        return Err("Insufficient funds".to_string());
    }

    wallet.balance -= amount;
    *wallet.ledger.entry(to).or_insert(0) += amount;

    Ok(())
}

#[update]
fn receive(from: String, amount: Token) {
    let wallet = ic_cdk::storage::get_mut::<Wallet>();

    wallet.balance += amount;
    *wallet.ledger.entry(from).or_insert(0) += amount;
}

#[query]
fn balance() -> Token {
    let wallet = ic_cdk::storage::get::<Wallet>();

    wallet.balance
}n main() {
    println!("Hello, world!");
}
