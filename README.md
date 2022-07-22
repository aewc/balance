# balance
```sh
dfx canister call balance multiple '(0, 131072)'
()

dfx canister call balance stablesize            
(101 : nat64)

dfx canister call balance multiple '(0, 4_915_200)'
Error: The Replica returned an error: code 5, message: "Canister rwlgt-iiaaa-aaaaa-aaaaa-cai exceeded the cycles limit for single message execution."

dfx canister call balance multiple '(0, 200_000)'
()
dfx canister call balance multiple '(200_000, 400_000)'
()
dfx canister call balance multiple '(400_000, 600_000)'
()
dfx canister call balance stablesize                
(460 : nat64)

# 460 * 65536 / 600_000 = 50.2442666667, means every u32(4 bytes) in StableBTreeMap need 50 bytes space.

# 4_915_200 * 50.2442666667 / 65536 = 376.8320000003, means if we want to store the 300Gib StableMemory's pages indexes, they need 377 pages, 24.69 MB
