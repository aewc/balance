use candid::{candid_method, Principal};
use ic_cdk::{
    api::stable,
    api::time,
    api::trap,
    caller,
    export::candid::{CandidType, Decode, Deserialize, Encode},
};
use ic_cdk_macros::*;
use ic_helpers::stable::{
    chunk_manager::VirtualMemory,
    export::stable_structures::{RestrictedMemory, StableBTreeMap},
    StableMemory,
};
use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;

const BALANCE_INDEX: u8 = 0;
const HISTORY_INDEX: u8 = 1;

#[derive(CandidType, Deserialize, Clone)]
struct History {
    id: u64,
    from: Principal,
    to: Principal,
    value: u64,
    timestamp: u64,
}

thread_local! {
    static BLAN: Rc<RefCell<StableBTreeMap<VirtualMemory<RestrictedMemory<StableMemory>, RestrictedMemory<StableMemory>>, Vec<u8>, Vec<u8>>>> = Rc::new(RefCell::new(StableBTreeMap::init(VirtualMemory::init(RestrictedMemory::new(StableMemory::default(), 20..131072), RestrictedMemory::new(StableMemory::default(), 0..20), BALANCE_INDEX), 29, 8)));
    static HIST: Rc<RefCell<StableBTreeMap<VirtualMemory<RestrictedMemory<StableMemory>, RestrictedMemory<StableMemory>>, Vec<u8>, Vec<u8>>>> = Rc::new(RefCell::new(StableBTreeMap::init(VirtualMemory::init(RestrictedMemory::new(StableMemory::default(), 20..131072), RestrictedMemory::new(StableMemory::default(), 0..20), HISTORY_INDEX), 121, 0)));
}

#[candid_method(query, rename = "balance_of")]
#[query(name = "balance_of")]
fn balance_of(owner: Principal) -> u64 {
    let owner = owner.as_slice().to_vec();
    BLAN.with(|s| {
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

#[candid_method(query, rename = "get_history")]
#[query(name = "get_history")]
fn get_history(from: u64, amount: u64) -> Vec<History> {
    let mut result = vec![];
    HIST.with(|s| {
        let state = s.borrow();
        for (i, (key, _)) in state.iter().enumerate() {
            let i = i as u64;
            if i >= from && i < from + amount {
                result.push(
                    Decode!(&key, History)
                        .unwrap_or_else(|e| trap(&format!("decode history error: {:?}", e))),
                );
            }
        }
    });
    result
}

#[candid_method(query, rename = "get_balance")]
#[query(name = "get_balance")]
fn get_balance(from: u64, amount: u64) -> Vec<(u64, u64)> {
    let mut res = vec![];
    BLAN.with(|s| {
        let state = s.borrow();
        for i in from..from + amount {
            let result = state.get(&i.to_be_bytes().to_vec());

            let a = match result {
                Some(v) => u64::from_be_bytes(
                    v.try_into()
                        .unwrap_or_else(|err| trap(&format!("from_be_bytes error: {:?}", err))),
                ),
                None => break,
            };

            res.push((i, a));
        }
    });
    res
}

#[candid_method(update, rename = "multiple")]
#[update(name = multiple)]
fn multiple1(start: u64, size: u64) -> Result<(), String> {
    BLAN.with(|b| {
        let mut state = b.borrow_mut();
        for i in start..(start + size) {
            state
                .insert(i.to_be_bytes().to_vec(), i.to_be_bytes().to_vec())
                .unwrap();
        }
    });
    HIST.with(|h| {
        let mut state = h.borrow_mut();
        for i in start..start + size {
            let id = state.len();
            let h = History {
                id,
                from: caller(),
                to: Principal::anonymous(),
                value: i,
                timestamp: time(),
            };
            let h_bytes = Encode!(&h).unwrap_or_else(|e| trap(&format!("encode! error: {}", e)));
            state.insert(h_bytes, vec![]).unwrap();
        }
    });
    Ok(())
}

#[candid_method(update, rename = "transfer")]
#[update(name = transfer)]
fn transfer(to: Principal, amount: u64) -> Result<(), String> {
    let caller = caller();
    let caller_slice = caller.as_slice().to_vec();
    let to_slice = to.as_slice().to_vec();
    let now = time();

    let caller_balance = balance_of(caller);
    let caller_balance_new = caller_balance
        .checked_sub(amount)
        .unwrap_or_else(|| trap(&format!("caller balance sub error.")));
    BLAN.with(|s| {
        let mut state = s.borrow_mut();
        state
            .insert(caller_slice, caller_balance_new.to_be_bytes().to_vec())
            .unwrap_or_else(|err| trap(&format!("insert caller error: {}", err)))
    });

    let to_balance = balance_of(to);
    let to_balance_new = to_balance
        .checked_add(amount)
        .unwrap_or_else(|| trap(&format!("to balance add error.")));
    BLAN.with(|s| {
        let mut state = s.borrow_mut();
        state
            .insert(to_slice, to_balance_new.to_be_bytes().to_vec())
            .unwrap_or_else(|err| trap(&format!("insert to error: {}", err)));
    });

    HIST.with(|h| {
        let mut history = h.borrow_mut();
        let id = history.len();
        let h = History {
            id,
            from: caller,
            to,
            value: amount,
            timestamp: now,
        };
        let h_bytes = Encode!(&h).unwrap_or_else(|e| trap(&format!("encode! error: {}", e)));
        history
            .insert(h_bytes, vec![])
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

    BLAN.with(|s| {
        let mut state = s.borrow_mut();
        state
            .insert(caller_slice, caller_balance_new.to_be_bytes().to_vec())
            .unwrap_or_else(|err| trap(&format!("insert caller balance error: {}", err)));
    });

    Ok(())
}

#[query(name = "read_raw_memory")]
#[candid_method(query, rename = "read_raw_memory")]
fn read_raw_memory(position: u64, size: u64) -> Vec<u8> {
    let mut buf = [0].repeat(size as usize);
    stable::stable64_read(position, &mut buf);
    return buf;
}

#[query(name = "page_info")]
#[candid_method(query, rename = "page_info")]
fn page_info1() -> (usize, Vec<u32>, usize, Vec<u32>) {
    let manager = StableBTreeMap::load(RestrictedMemory::new(StableMemory::default(), 0..20));
    let a = manager
        .range(vec![BALANCE_INDEX], None)
        .map(|a: (Vec<u8>, Vec<u8>)| decode(BALANCE_INDEX, a.0.try_into().expect("decode error")))
        .collect::<Vec<_>>();
    let b = manager
        .range(vec![HISTORY_INDEX], None)
        .map(|a| decode(HISTORY_INDEX, a.0.try_into().expect("decode error")))
        .collect::<Vec<_>>();
    (a.len(), a, b.len(), b)
}

fn decode(index: u8, bytes: [u8; 4]) -> u32 {
    assert!(bytes[0] == index);
    let mut bytes = bytes;
    bytes[0] = 0;
    u32::from_be_bytes(bytes)
}

#[query(name = "stablesize")]
#[candid_method(query, rename = "stablesize")]
fn stable_size() -> u64 {
    stable::stable64_size()
}

#[update(name = "grow")]
#[candid_method(update, rename = "grow")]
fn grow(pages: u64) -> u64 {
    stable::stable64_grow(pages).unwrap()
}

#[update(name = "write")]
#[candid_method(update, rename = "write")]
fn write(offset: u64, src: Vec<u8>) {
    stable::stable64_write(offset, &src)
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
