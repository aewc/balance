# balance
```sh
dfx canister --network ic call balance balance_of '(principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae")'
(0 : nat64)

dfx canister --network ic call balance balance_of '(principal "aaaaa-aa")'
(0 : nat64)

dfx canister --network ic call balance mint_it '(1000_000)'                  
(variant { Ok })

dfx canister --network ic call balance balance_of '(principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae")'
(1_000_000 : nat64)

dfx canister --network ic call balance transfer '(principal "aaaaa-aa", 1_000)'   
(variant { Ok })

dfx canister --network ic call balance balance_of '(principal "aaaaa-aa")'
(1_000 : nat64)

dfx canister --network ic call balance balance_of '(principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae")'
(999_000 : nat64)

# change the mint_it to mint and upgrade

cargo run > src/balance/balance.did 

dfx build --network ic

dfx canister --network ic install balance -m=upgrade

dfx canister --network ic call balance balance_of '(principal "aaaaa-aa")'
(0 : nat64)

dfx canister --network ic call balance balance_of '(principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae")'
(0 : nat64)

# failed, need to figure out: 1. The StableBtreeMap don't save when every entry modified. 2. The reload failed, maybe the StableStorage shouldn't new.
```