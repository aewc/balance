use candid::candid_method;
use ic_cdk::{api::stable, trap};
use ic_cdk_macros::*;
use ic_helpers::stable::{export::stable_structures::StableBTreeMap, StableMemory};
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    static INDEX: Rc<RefCell<StableBTreeMap<StableMemory, Vec<u8>, Vec<u8>>>> = Rc::new(RefCell::new(StableBTreeMap::init(StableMemory::default(), 4, 0)));
}

#[update(name = "multiple")]
#[candid_method(update, rename = "multiple")]
fn multiple(from: u32, to: u32) {
    INDEX.with(|s| {
        let mut state = s.borrow_mut();
        for i in from..to {
            state
                .insert(i.to_be_bytes().to_vec(), vec![])
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
fn stable_size() -> u64 {
    stable::stable64_size()
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    candid::export_service!();
    std::print!("{}", __export_service());
}
