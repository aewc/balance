# balance
```sh
cargo run > src/balance/balance.did 

dfx build --network ic 

dfx canister --network ic install balance -m=reinstall

dfx canister --network ic call balance stablesize
(1 : nat64)

dfx canister --network ic status balance
Canister status call result for balance.
Status: Running
Controllers: xxx
Memory allocation: 0
Compute allocation: 0
Freezing threshold: 2_592_000
Memory Size: Nat(1792174)
Balance: 1_482_427_534_392 Cycles
Module hash: 0xf9d749f62a49df2ef4563664d0cc75961c1e53bbd0be8c40633097864075f6f4
# the wasm bytes code is 470KB, and the whole memory is 1.79MB. now the stable storage is almost 0B, so the wasm heap is used 1.79MB, the other things used 1.32MB.

dfx wallet --network ic send kg7yl-7aaaa-aaaag-qagfq-cai 5000000000000

dfx canister --network ic status balance
Memory allocation: 0
Compute allocation: 0
Freezing threshold: 2_592_000
Memory Size: Nat(1792174)
Balance: 6_482_425_568_864 Cycles
Module hash: 0xf9d749f62a49df2ef4563664d0cc75961c1e53bbd0be8c40633097864075f6f4

dfx canister --network ic call balance balance_of '(principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae")'
(0 : nat64)

dfx canister --network ic call balance balance_of '(principal "aaaaa-aa")'
(0 : nat64)

dfx canister --network ic call balance mint '(1000_000)'                  
(variant { Ok })

dfx canister --network ic call balance balance_of '(principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae")'
(1_000_000 : nat64)

dfx canister --network ic call balance transfer '(principal "aaaaa-aa", 1_000)'   
(variant { Ok })

dfx canister --network ic call balance balance_of '(principal "aaaaa-aa")'
(1_000 : nat64)

dfx canister --network ic call balance balance_of '(principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae")'
(999_000 : nat64)
```

test upgrade:
```sh
# modify the src code to change the module hash to upgrade

dfx build --network ic

dfx canister --network ic install balance -m=upgrade

dfx canister --network ic call balance balance_of '(principal "aaaaa-aa")'
(1_000 : nat64)

dfx canister --network ic call balance balance_of '(principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae")'
(999_000 : nat64)

# Success
```

test large size:
```sh
dfx canister --network ic call balance stablesize
(1 : nat64) # only one page size

dfx canister --network ic call balance read_raw_memory '(0, 300)'
(
  blob "BTR\01\1d\00\00\00\08\00\00\00t\00\00\00\00\00\00\00\02\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00BTA\01\00\00\00\00V\02\00\00\00\00\00\00\01\00\00\00\00\00\00\00\ca\02\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00CHK\01\01\00\00\00\00\00\00\00\00\00\00\00BTN\01\00\02\00\00\00\00\00\98\ef\18\ac\8c\0c\d7\8e,)a\d8\f3\9e-\1a\c2\a0\a8\few\19\ef\82\d7B\d1\a6\02\08\00\00\00\00\00\00\00\00\00\03\e8\1d\00\00\00\98\ef\18\ac\8c\0c\d7\8e,)a\d8\f3\9e-\1a\c2\a0\a8\few\19\ef\82\d7B\d1\a6\02\08\00\00\00\00\00\00\00\00\0f>X\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00",
)

dfx canister --network ic call balance read_raw_memory '(0, 65537)'
Error: The Replica returned an error: code 5, message: "IC0502: Canister kg7yl-7aaaa-aaaag-qagfq-cai trapped: stable memory out of bounds"
# as there are only 1 page in stable storage, the max is (0, 65536)

# input 50k entries, failed
dfx canister --network ic call balance multiple '(1, 50_000)'
Error: The Replica returned an error: code 5, message: "Canister kg7yl-7aaaa-aaaag-qagfq-cai exceeded the instruction limit for single message execution."

dfx canister --network ic call balance multiple '(1, 10_000)'
()

dfx canister --network ic call balance stablesize             
(19 : nat64)

# Memory Size: Nat(2971862)
# Balance: 6_457_294_077_450 Cycles

dfx canister --network ic call balance multiple '(1, 10_000)'
()

dfx canister --network ic call balance multiple '(10_000, 30_000)'
()

# Memory Size: Nat(5462230)
# Balance: 6_451_537_186_802 Cycles

dfx canister --network ic call balance stablesize                 
(57 : nat64)

30000 nodes ~ 57 pages ~ 3_735_552 bytes 
1 nodes ~ 124.5 bytes (key 29, value 8)

3.73MB is used in stable storage, the whole used is 5.46MB, so 1.73MB is used in wasm heap, almost the same with 1.79MB, so the wasm heap didn't used when store data in stable storage.
```