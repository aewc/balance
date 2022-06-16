use candid::{candid_method, Principal};
use ic_cdk::{api::trap, caller};
use ic_cdk_macros::*;
use stable_structures::{stable_storage::StableStorage, StableBTreeMap};
use std::cell::RefCell;
use std::convert::TryInto;

thread_local! {
    static STATE: RefCell<StableBTreeMap<StableStorage>> = RefCell::new(StableBTreeMap::new(StableStorage::new(), 29, 8));
}

#[candid_method(query)]
#[query]
fn balance_of(owner: Principal) -> u64 {
    let owner = owner.as_slice().to_vec();
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

#[candid_method(update)]
#[update]
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
            .unwrap_or_else(|err| trap(&format!("insert to error: {}", err)))
    });
    Ok(())
}

#[candid_method(update)]
#[update]
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
            .unwrap_or_else(|err| trap(&format!("insert caller balance error: {}", err)))
    });

    Ok(())
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    candid::export_service!();
    std::print!("{}", __export_service());
}
