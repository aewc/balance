use candid::candid_method;
use ic_cdk::api::{
    stable::{stable64_grow, stable64_size, stable64_write},
    trap,
};
use ic_cdk_macros::*;
use stable_structures::{
    stable_storage::StableStorage, Memory, RestrictedMemory, StableBTreeMap, VectorMemory,
};
use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;

const WASM_PAGE_SIZE: u64 = 65536;

// Stable memory:
// 0     - 65536  page(0-4GB) for StableBTreeMap(SBTM)
// 65536 - 131072 page(4-8GB) for Linear Data(SVEC)
//
// Wasm heap:
// wasm heap btree map(WHBTM) and wasm heap vec(WHVEC) share the 4GB wasm heap storage
thread_local! {
    static SBTM: RefCell<StableBTreeMap<RestrictedMemory<StableStorage>>> = RefCell::new(StableBTreeMap::load(RestrictedMemory::new(StableStorage::default(), 0..65536)));
    static SVEC: RefCell<RestrictedMemory<StableStorage>> = RefCell::new(RestrictedMemory::new(StableStorage::default(), 65536..131072));

    static WHBTM: RefCell<StableBTreeMap<VectorMemory>> = RefCell::new(StableBTreeMap::new(Rc::new(RefCell::new(Vec::new())), 8, 8));
    static WHVEC: RefCell<Vec<u8>> = RefCell::new(Vec::new());
}

#[init]
#[candid_method(init)]
fn init() {
    StableBTreeMap::new(
        RestrictedMemory::new(StableStorage::default(), 0..65536),
        8,
        8,
    );
}

#[update(name = "write_map_stable")]
#[candid_method(update, rename = "write_map_stable")]
fn write_map_stable(from: u64, to: u64) {
    SBTM.with(|s| {
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

#[update(name = "write_vec_stable")]
#[candid_method(update, rename = "write_vec_stable")]
fn write_vec_stable(start: u64, size: u64) {
    SVEC.with(|s| {
        let total = start + size;
        let svec = s.borrow();
        let current_size = svec.size();
        let pages = (total + WASM_PAGE_SIZE - 1) / WASM_PAGE_SIZE;

        if pages > current_size {
            let pages_to_grow = pages - current_size;
            let result = svec.grow(pages - current_size);
            if result == -1 {
                trap(&format!(
                    "failed to grow stable memory by {} pages",
                    pages_to_grow
                ))
            }
        }
        let buf = [1u8].repeat(size as usize);
        svec.write(start, &buf);
    });
}

#[update(name = "write_map_wasm_heap")]
#[candid_method(update, rename = "write_map_wasm_heap")]
fn write_map_wasm_heap(from: u64, to: u64) {
    WHBTM.with(|s| {
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

#[update(name = "write_vec_wasm_heap")]
#[candid_method(update, rename = "write_vec_wasm_heap")]
fn write_vec_wasm_heap(start: u64, size: u64) {
    WHVEC.with(|s| {
        let mut whvec = s.borrow_mut();
        let len = whvec.len() as u64;
        if len < start + size {
            let new = start + size;
            whvec.resize(new as usize, 1u8);
        }
        if start < len {
            for i in start..len {
                whvec[i as usize] = 1;
            }
        }
    });
}

#[query(name = "read_map_stable")]
#[candid_method(query, rename = "read_map_stable")]
fn read_map_stable(from: u64, to: u64) -> u64 {
    let mut buf = vec![];
    SBTM.with(|b| {
        let state = b.borrow();
        for i in from..to {
            let value = state.get(&i.to_be_bytes().to_vec());
            if value.is_none() {
                break;
            }
            let value = value.unwrap_or_else(|| trap(&format!("read_map_stable error")));
            buf.push(u64::from_be_bytes(value.try_into().unwrap_or_else(|e| {
                trap(&format!("read_map_stable error {:?}", e))
            })));
        }
    });
    buf.len() as u64
}

#[query(name = "read_vec_stable")]
#[candid_method(query, rename = "read_vec_stable")]
fn read_vec_stable(position: u64, size: u64) -> u64 {
    let mut buf = [0u8].repeat(size as usize);
    SVEC.with(|s| {
        s.borrow().read(position, &mut buf);
    });
    buf.len() as u64
}

#[query(name = "read_map_wasm_heap")]
#[candid_method(query, rename = "read_map_wasm_heap")]
fn read_map_wasm_heap(from: u64, to: u64) -> u64 {
    let mut buf = vec![];
    WHBTM.with(|b| {
        let state = b.borrow();
        for i in from..to {
            let value = state.get(&i.to_be_bytes().to_vec());
            if value.is_none() {
                break;
            }
            let value = value.unwrap_or_else(|| trap(&format!("read_map_stable error")));
            buf.push(u64::from_be_bytes(value.try_into().unwrap_or_else(|e| {
                trap(&format!("read_map_stable error {:?}", e))
            })));
        }
    });
    buf.len() as u64
}

#[query(name = "read_vec_wasm_heap")]
#[candid_method(query, rename = "read_vec_wasm_heap")]
fn read_vec_wasm_heap(position: u64, size: u64) -> u64 {
    let position = position as usize;
    let size = size as usize;
    let mut buf = vec![];
    WHVEC.with(|b| {
        let state = b.borrow();
        buf = state[position..position + size].to_vec();
    });
    buf.len() as u64
}

#[update]
#[candid_method(update)]
fn addsize(total: u64) -> u64 {
    let current_size = stable64_size();
    let pages = (total + WASM_PAGE_SIZE - 1) / WASM_PAGE_SIZE;
    if pages > current_size {
        let pages_to_grow = pages - current_size;
        let result = stable64_grow(pages - current_size);
        if result.is_err() {
            trap(&format!(
                "failed to grow stable memory by {} pages",
                pages_to_grow
            ))
        }
    }
    let buf: Vec<u8> = Vec::with_capacity(total as usize);
    stable64_write(0, &buf);
    pages
}

#[query(name = "stablesize")]
#[candid_method(query, rename = "stablesize")]
fn stable_size() -> u64 {
    stable64_size()
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
