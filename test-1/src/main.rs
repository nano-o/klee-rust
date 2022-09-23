#![no_std]
use soroban_sdk::{contractimpl, symbol, vec, Env, Symbol, Vec, BytesN};
//use super::{Contract, ContractClient};


pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol!("Hello"), to]
    }
}

fn main() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let words = client.hello(&symbol!("Dev"));
    assert_eq!(
        words,
        vec![&env, symbol!("Hello"), symbol!("Dev"),]
        );
}
