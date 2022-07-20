use candid::candid_method;
use ic_cdk::api::stable;
use ic_cdk_macros::*;

#[query(name = "read")]
#[candid_method(query, rename = "read")]
fn read(position: u64, size: u64) -> Vec<u8> {
    let mut buf = [0].repeat(size as usize);
    stable::stable64_read(position, &mut buf);
    return buf;
}

#[query(name = "size")]
#[candid_method(query, rename = "size")]
fn size() -> u64 {
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

#[update(name = "grow_panic")]
#[candid_method(update, rename = "grow_panic")]
fn grow_panic(pages: u64) -> u64 {
    let size = stable::stable64_grow(pages).unwrap();
    return_err().expect("error");
    size
}

#[update(name = "write_panic")]
#[candid_method(update, rename = "write_panic")]
fn write_panic(offset: u64, src: Vec<u8>) {
    stable::stable64_write(offset, &src);
    return_err().expect("error");
    assert!(false);
}

fn return_err() -> Result<(), String> {
    Err("test".to_string())
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    candid::export_service!();
    std::print!("{}", __export_service());
}
