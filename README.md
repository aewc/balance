# stable

term 0:
```sh
dfx start --clean
```

term 1:
```sh
dfx deploy --no-wallet

dfx canister call balance size
(0 : nat64)

dfx canister call balance grow '(1)'
(0 : nat64)

dfx canister call balance size
(1 : nat64)

dfx canister call balance grow_panic '(1)'
Error: The Replica returned an error: code 5, message: "Canister rwlgt-iiaaa-aaaaa-aaaaa-cai trapped explicitly: Panicked at 'error: "test"', src/balance/src/main.rs:35:18"

dfx canister call balance size
(1 : nat64)

dfx canister call balance grow '(1)'
(1 : nat64)

dfx canister call balance size
(2 : nat64)

dfx canister call balance grow_panic '(1)'
Error: The Replica returned an error: code 5, message: "Canister rwlgt-iiaaa-aaaaa-aaaaa-cai trapped explicitly: Panicked at 'error: "test"', src/balance/src/main.rs:35:18"

dfx canister call balance size
(2 : nat64)


dfx canister call balance read '(0, 10)'
(blob "\00\00\00\00\00\00\00\00\00\00")

dfx canister call balance write '(0, vec {1;1;1;1;1;1;1;1;1;1;})'
()

dfx canister call balance read '(0, 10)'
(blob "\01\01\01\01\01\01\01\01\01\01")

dfx canister call balance write_panic '(0, vec {2;2;2;2;2;2;2;2;2;2;})'
Error: The Replica returned an error: code 5, message: "Canister rwlgt-iiaaa-aaaaa-aaaaa-cai trapped explicitly: Panicked at 'error: "test"', src/balance/src/main.rs:43:18"

dfx canister call balance read '(0, 10)'
(blob "\01\01\01\01\01\01\01\01\01\01")
```
