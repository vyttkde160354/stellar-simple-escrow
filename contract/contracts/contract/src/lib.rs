#![no_std]

use soroban_sdk::{
    contract, contractimpl, symbol_short, Address, Env, Symbol
};

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {

    // 🔒 INIT ESCROW
    pub fn init(
        env: Env,
        payer: Address,
        payee: Address,
        amount: i128,
    ) {
        payer.require_auth();

        env.storage().instance().set(&symbol_short!("PAYER"), &payer);
        env.storage().instance().set(&symbol_short!("PAYEE"), &payee);
        env.storage().instance().set(&symbol_short!("AMOUNT"), &amount);
        env.storage().instance().set(&symbol_short!("STATE"), &symbol_short!("LOCK"));
    }

    // ✅ RELEASE tiền cho payee
    pub fn release(env: Env, payer: Address) -> Symbol {
        payer.require_auth();

        let stored_payer: Address = env
            .storage()
            .instance()
            .get(&symbol_short!("PAYER"))
            .unwrap();

        if payer != stored_payer {
            panic!("Not authorized");
        }

        env.storage().instance().set(&symbol_short!("STATE"), &symbol_short!("DONE"));

        symbol_short!("released")
    }

    // 🔄 REFUND tiền lại payer
    pub fn refund(env: Env, payer: Address) -> Symbol {
        payer.require_auth();

        let stored_payer: Address = env
            .storage()
            .instance()
            .get(&symbol_short!("PAYER"))
            .unwrap();

        if payer != stored_payer {
            panic!("Not authorized");
        }

        env.storage().instance().set(&symbol_short!("STATE"), &symbol_short!("REFUND"));

        symbol_short!("refunded")
    }

    // 📊 CHECK STATE
    pub fn get_state(env: Env) -> Symbol {
        env.storage()
            .instance()
            .get(&symbol_short!("STATE"))
            .unwrap()
    }
}
stellar contract invoke \
  --id CBP2HXD7R2QZ5K22N42776PWILYNUHKB3JWDQJ4R3D75H5RLHWMUDQSU \
  --source student \
  --network testnet \
  -- \
  init \
  --payer student \
  --payee GBHR5NW4QVKZEWSEYD6R2CS5F2TIQJNRWM6W4NSLLHKX2FD4NBDDINP2 \
  --amount 100

  stellar contract invoke \
  --id CBP2HXD7R2QZ5K22N42776PWILYNUHKB3JWDQJ4R3D75H5RLHWMUDQSU \
  --source student \
  --network testnet \
  -- \
  get_state

  stellar contract invoke \
  --id CBP2HXD7R2QZ5K22N42776PWILYNUHKB3JWDQJ4R3D75H5RLHWMUDQSU \
  --source student \
  --network testnet \
  -- \
  release \
  --payer student