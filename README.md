# balance
```sh
cargo run > src/balance/balance.did 

dfx build --network ic 

dfx canister --network ic install balance -m=reinstall

dfx canister --network ic call balance stablesize
(8_193 : nat64)

dfx canister --network ic status balance
Canister status call result for balance.
Status: Running
Controllers: xxx
Memory allocation: 0
Compute allocation: 0
Freezing threshold: 2_592_000
Memory Size: Nat(538715166)
Balance: 6_450_003_282_209 Cycles
Module hash: 0x3640d30eecca9fb4613189fae01e747c41e7a79bb5236afb2c8a4f1207be7bca
# the whole memory is 514 MB. now the stable storage is 512 MB, 

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

dfx canister --network ic call balance transfer '(principal "ktfx3-4dj7o-f4lqf-gab56-fgkuw-aagt6-jzpkd-o7xzp-f6a3p-nm6wl-wae", 2_000)'   
(variant { Ok })

dfx canister --network ic call balance transfer '(principal "yd5hv-nayum-igkpt-jtrhr-aqfqq-pfkxn-fyxvh-yryn3-rez4o-p6vks-hqe", 3_000)'   
(variant { Ok })

dfx canister --network ic call balance transfer '(principal "qbyxf-lll3o-rmkt4-thocb-vo3bz-d2mfj-3nwa7-pv2rf-iqfqw-rgkg2-hqe", 4_000)'   
(variant { Ok })

dfx canister --network ic call balance balance_of '(principal "aaaaa-aa")'
(1_000 : nat64)

dfx canister --network ic call balance balance_of '(principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae")'
(990_000 : nat64)

dfx canister --network ic call balance get_history '(0, 10)'
(
  vec {
    record {
      id = 0 : nat64;
      to = principal "aaaaa-aa";
      value = 1_000 : nat64;
      from = principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      timestamp = 1_655_716_907_965_519_236 : nat64;
    };
    record {
      id = 1 : nat64;
      to = principal "ktfx3-4dj7o-f4lqf-gab56-fgkuw-aagt6-jzpkd-o7xzp-f6a3p-nm6wl-wae";
      value = 2_000 : nat64;
      from = principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      timestamp = 1_655_716_929_637_200_174 : nat64;
    };
    record {
      id = 2 : nat64;
      to = principal "yd5hv-nayum-igkpt-jtrhr-aqfqq-pfkxn-fyxvh-yryn3-rez4o-p6vks-hqe";
      value = 3_000 : nat64;
      from = principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      timestamp = 1_655_716_936_103_400_309 : nat64;
    };
    record {
      id = 3 : nat64;
      to = principal "qbyxf-lll3o-rmkt4-thocb-vo3bz-d2mfj-3nwa7-pv2rf-iqfqw-rgkg2-hqe";
      value = 4_000 : nat64;
      from = principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      timestamp = 1_655_716_944_246_023_692 : nat64;
    };
  },
)
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

dfx canister --network ic call balance get_history '(0, 10)'
(
  vec {
    record {
      id = 0 : nat64;
      to = principal "aaaaa-aa";
      value = 1_000 : nat64;
      from = principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      timestamp = 1_655_716_907_965_519_236 : nat64;
    };
    record {
      id = 1 : nat64;
      to = principal "ktfx3-4dj7o-f4lqf-gab56-fgkuw-aagt6-jzpkd-o7xzp-f6a3p-nm6wl-wae";
      value = 2_000 : nat64;
      from = principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      timestamp = 1_655_716_929_637_200_174 : nat64;
    };
    record {
      id = 2 : nat64;
      to = principal "yd5hv-nayum-igkpt-jtrhr-aqfqq-pfkxn-fyxvh-yryn3-rez4o-p6vks-hqe";
      value = 3_000 : nat64;
      from = principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      timestamp = 1_655_716_936_103_400_309 : nat64;
    };
    record {
      id = 3 : nat64;
      to = principal "qbyxf-lll3o-rmkt4-thocb-vo3bz-d2mfj-3nwa7-pv2rf-iqfqw-rgkg2-hqe";
      value = 4_000 : nat64;
      from = principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      timestamp = 1_655_716_944_246_023_692 : nat64;
    };
  },
)

# Success
```

test large size:
```sh
dfx canister --network ic call balance stablesize
(8_193 : nat64)

dfx canister --network ic call balance read_raw_memory '(536_870_912, 300)'
(
  blob "BTR\01\08\00\00\00\96\00\00\00t\00\00\00\00\00\00\00\04\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00BTA\01\00\00\00\00\89\07\00\00\00\00\00\00\01\00\00\00\00\00\00\00\fd\07\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00CHK\01\01\00\00\00\00\00\00\00\00\00\00\00BTN\01\00\04\00\08\00\00\00\00\00\00\00\00\00\00\00\5c\00\00\00DIDL\01l\05\db\b7\01x\fb\ca\01h\f1\fe\e1\8d\03x\ea\ca\8a\9e\04h\d6\a9\bb\ae\0ax\01\00\00\00\00\00\00\00\00\00\01\00\e8\03\00\00\00\00\00\00\01\1d\98\ef\18\ac\8c\0c\d7\8e,)a\d8\f3\9e-\1a\c2\a0\a8\few\19\ef\82\d7B\d1\a6\02\84\95\e4c\c1I\fa\16\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\00\00\00",
)
```