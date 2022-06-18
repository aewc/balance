use candid::{candid_method, Principal};
use ic_cdk::{api::stable, api::trap, caller};
use ic_cdk_macros::*;
use stable_structures::{stable_storage::StableStorage, StableBTreeMap};
use std::cell::RefCell;
use std::convert::TryInto;

thread_local! {
    static STATE: RefCell<StableBTreeMap<StableStorage>> = RefCell::new(StableBTreeMap::load(StableStorage::default()));
}

#[init]
#[candid_method(init)]
fn init() {
    StableBTreeMap::new(StableStorage::default(), 29, 8);
}

#[candid_method(query, rename = "balance_of")]
#[query(name = "balance_of")]
fn balance_of(owner1: Principal) -> u64 {
    let owner = owner1.as_slice().to_vec();
    STATE.with(|s| {
        let state = s.borrow();
        let result = state.get(&owner);
        match result {
            Some(v) => u64::from_be_bytes(
                v.try_into()
                    .unwrap_or_else(|err| trap(&format!("from_be_bytes error: {:?}", err))),
            ),
            None => 0,
        }
    })
}

#[candid_method(update, rename = "transfer")]
#[update(name = transfer)]
fn transfer(to: Principal, amount: u64) -> Result<(), String> {
    let caller = caller();
    let caller_slice = caller.as_slice().to_vec();
    let to_slice = to.as_slice().to_vec();

    let caller_balance = balance_of(caller);
    let caller_balance_new = caller_balance
        .checked_sub(amount)
        .unwrap_or_else(|| trap(&format!("caller balance sub error.")));
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state
            .insert(caller_slice, caller_balance_new.to_be_bytes().to_vec())
            .unwrap_or_else(|err| trap(&format!("insert caller error: {}", err)))
    });

    let to_balance = balance_of(to);
    let to_balance_new = to_balance
        .checked_add(amount)
        .unwrap_or_else(|| trap(&format!("to balance add error.")));
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state
            .insert(to_slice, to_balance_new.to_be_bytes().to_vec())
            .unwrap_or_else(|err| trap(&format!("insert to error: {}", err)));
    });
    Ok(())
}

#[candid_method(update, rename = "mint")]
#[update(name = "mint")]
fn mint(amount: u64) -> Result<(), String> {
    let caller = caller();
    let caller_slice = caller.as_slice().to_vec();
    let caller_balance = balance_of(caller);
    let caller_balance_new = caller_balance
        .checked_add(amount)
        .unwrap_or_else(|| trap(&format!("caller balance add error.")));

    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state
            .insert(caller_slice, caller_balance_new.to_be_bytes().to_vec())
            .unwrap_or_else(|err| trap(&format!("insert caller balance error: {}", err)));
    });

    Ok(())
}

#[update(name = "multiple")]
#[candid_method(update, rename = "multiple")]
fn multiple(from: u64, to: u64) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        for i in from..=to {
            state
                .insert(
                    i.to_be_bytes().to_vec(),
                    (u64::MAX - i).to_be_bytes().to_vec(),
                )
                .unwrap_or_else(|err| trap(&format!("insert multiple error: {}", err)));
        }
    });
}

#[query(name = "read_raw_memory")]
#[candid_method(query, rename = "read_raw_memory")]
fn read_raw_memory(position: u64, size: u64) -> Vec<u8> {
    let mut buf = [0].repeat(size as usize);
    stable::stable64_read(position, &mut buf);
    return buf;
}

#[query(name = "stablesize")]
#[candid_method(query, rename = "stablesize")]
fn stable_size_t() -> u64 {
    stable::stable64_size()
}

#[update(name = "wallet_receive")]
#[candid_method(update, rename = "wallet_receive")]
fn wallet_receive() {
    let cycles = ic_cdk::api::call::msg_cycles_available128();
    ic_cdk::api::call::msg_cycles_accept128(cycles);
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    candid::export_service!();
    std::print!("{}", __export_service());
}
