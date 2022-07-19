# balance
```sh
cargo run > src/balance/balance.did 

dfx build 

dfx canister install balance -m=reinstall

dfx canister call balance stablesize
(0 : nat64)

dfx canister status balance

Canister status call result for balance.
Status: Running
Controllers: yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae
Memory allocation: 0
Compute allocation: 0
Freezing threshold: 2_592_000
Memory Size: Nat(3176316)
Balance: 100_000_000_000_000 Cycles
Module hash: 0x01b92963de36507e1c666334ee18840df118398fb638c5c3dc4689ad46356eec

dfx canister call balance balance_of '(principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae")'
(0 : nat64)

dfx canister call balance balance_of '(principal "aaaaa-aa")'
(0 : nat64)

dfx canister call balance mint '(1000_000)'
(variant { Ok })

dfx canister call balance balance_of '(principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae")'
(1_000_000 : nat64)

dfx canister call balance transfer '(principal "aaaaa-aa", 1_000)'   
(variant { Ok })

dfx canister call balance transfer '(principal "ktfx3-4dj7o-f4lqf-gab56-fgkuw-aagt6-jzpkd-o7xzp-f6a3p-nm6wl-wae", 2_000)'   
(variant { Ok })

dfx canister call balance transfer '(principal "yd5hv-nayum-igkpt-jtrhr-aqfqq-pfkxn-fyxvh-yryn3-rez4o-p6vks-hqe", 3_000)'   
(variant { Ok })

dfx canister call balance transfer '(principal "qbyxf-lll3o-rmkt4-thocb-vo3bz-d2mfj-3nwa7-pv2rf-iqfqw-rgkg2-hqe", 4_000)'   
(variant { Ok })

dfx canister call balance balance_of '(principal "aaaaa-aa")'
(1_000 : nat64)

dfx canister call balance balance_of '(principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae")'
(990_000 : nat64)

dfx canister call balance get_history '(0, 10)'
(
  vec {
    record {
      id = 0 : nat64;
      to = principal "aaaaa-aa";
      value = 1_000 : nat64;
      from = principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      timestamp = 1_658_134_372_048_192_000 : nat64;
    };
    record {
      id = 1 : nat64;
      to = principal "ktfx3-4dj7o-f4lqf-gab56-fgkuw-aagt6-jzpkd-o7xzp-f6a3p-nm6wl-wae";
      value = 2_000 : nat64;
      from = principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      timestamp = 1_658_134_377_588_432_000 : nat64;
    };
    record {
      id = 2 : nat64;
      to = principal "yd5hv-nayum-igkpt-jtrhr-aqfqq-pfkxn-fyxvh-yryn3-rez4o-p6vks-hqe";
      value = 3_000 : nat64;
      from = principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      timestamp = 1_658_134_384_539_431_000 : nat64;
    };
    record {
      id = 3 : nat64;
      to = principal "qbyxf-lll3o-rmkt4-thocb-vo3bz-d2mfj-3nwa7-pv2rf-iqfqw-rgkg2-hqe";
      value = 4_000 : nat64;
      from = principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      timestamp = 1_658_134_398_434_299_000 : nat64;
    };
  },
)
```

test upgrade:
```sh
# modify the src code to change the module hash to upgrade

dfx build --network ic

dfx canister install balance -m=upgrade

dfx canister call balance balance_of '(principal "aaaaa-aa")'
(1_000 : nat64)

dfx canister call balance balance_of '(principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae")'
(999_000 : nat64)

dfx canister call balance get_history '(0, 10)'
(
  vec {
    record {
      id = 0 : nat64;
      to = principal "aaaaa-aa";
      value = 1_000 : nat64;
      from = principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      timestamp = 1_658_134_372_048_192_000 : nat64;
    };
    record {
      id = 1 : nat64;
      to = principal "ktfx3-4dj7o-f4lqf-gab56-fgkuw-aagt6-jzpkd-o7xzp-f6a3p-nm6wl-wae";
      value = 2_000 : nat64;
      from = principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      timestamp = 1_658_134_377_588_432_000 : nat64;
    };
    record {
      id = 2 : nat64;
      to = principal "yd5hv-nayum-igkpt-jtrhr-aqfqq-pfkxn-fyxvh-yryn3-rez4o-p6vks-hqe";
      value = 3_000 : nat64;
      from = principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      timestamp = 1_658_134_384_539_431_000 : nat64;
    };
    record {
      id = 3 : nat64;
      to = principal "qbyxf-lll3o-rmkt4-thocb-vo3bz-d2mfj-3nwa7-pv2rf-iqfqw-rgkg2-hqe";
      value = 4_000 : nat64;
      from = principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      timestamp = 1_658_134_398_434_299_000 : nat64;
    };
  },
)

# Success
```

test large size:
```sh
dfx canister call balance stablesize
(22 : nat64)

# Page [0, 20) belongs to the Vistual Memory Manger. 
# In Vistual Memory Manger's Record, the page 0(in RestrictedMemory(20..131072), so it is page 20 in the whole stable memory) belongs to balance, and its struct's index = 0:
# [0, 0, 0, 0]
#  |  
# balance index
#
# the page 1 belongs to history, and its struct's index = 1:
# [1, 0, 0, 1]
#  | 
# history index
# so we can find them in \01\00\00\01\ below:
dfx canister call balance read_raw_memory '(0, 300)'
(
  blob "BTR\01\04\00\00\00\00\00\00\00t\00\00\00\00\00\00\00\02\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00BTA\01\00\00\00\00\eb\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00_\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00CHK\01\01\00\00\00\00\00\00\00\00\00\00\00BTN\01\00\02\00\04\00\00\00\00\00\00\00\00\00\00\00\04\00\00\00\01\00\00\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00",
)
# and it begins with "BTR", that means it is the begin of one btreemap.

# get the balance raw:
# 20 * 65536 = 1,310,720
dfx canister call balance read_raw_memory '(1_310_720, 300)'
(
  blob "BTR\01\1d\00\00\00\08\00\00\00t\00\00\00\00\00\00\00\05\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00BTA\01\00\00\00\00V\02\00\00\00\00\00\00\01\00\00\00\00\00\00\00\ca\02\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00CHK\01\01\00\00\00\00\00\00\00\00\00\00\00BTN\01\00\05\00\00\00\00\00\98\ef\18\ac\8c\0c\d7\8e,)a\d8\f3\9e-\1a\c2\a0\a8\few\19\ef\82\d7B\d1\a6\02\08\00\00\00\00\00\00\00\00\00\03\e8\1d\00\00\00\18\a3\10e>i\9cO\10@\b0\83\ca\ab\b4\b8\bdO\88\e1\bb\893\c7?\d5T\8f\02\08\00\00\00\00\00\00\00\00\00\0b\b8\1d\00\00\00i\fb\8b\c5\c0\a6\00{\e2\99T\b0\00i\f99z\86\ef\df//\81\b7\b5\9e\b2\ec\02\08\00\00\00\00\00\00\00\00\00\07\d0\1d\00\00\00k\db\a2\c5O\93;\84\1a\bba\c8\f4\c2\a7m\b0>\fa\ea%D\0b\0bD\ca6\8f\02\08\00\00\00\00\00\00\00\00",
)
# and it begins with "BTR", that means it is the begin of one btreemap.

# get the history raw:
# 21 * 65536 = 1,376,256
dfx canister call balance read_raw_memory '(1_376_256, 300)'
(
  blob "BTR\01y\00\00\00\00\00\00\00t\00\00\00\00\00\00\00\04\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00BTA\01\00\00\00\00\f2\05\00\00\00\00\00\00\01\00\00\00\00\00\00\00f\06\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00CHK\01\01\00\00\00\00\00\00\00\00\00\00\00BTN\01\00\04\00\5c\00\00\00DIDL\01l\05\db\b7\01x\fb\ca\01h\f1\fe\e1\8d\03x\ea\ca\8a\9e\04h\d6\a9\bb\ae\0ax\01\00\00\00\00\00\00\00\00\00\01\00\e8\03\00\00\00\00\00\00\01\1d\98\ef\18\ac\8c\0c\d7\8e,)a\d8\f3\9e-\1a\c2\a0\a8\few\19\ef\82\d7B\d1\a6\02\00B\cc\1bm\e0\02\17\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00y\00\00\00DIDL\01l\05\db\b7\01x\fb\ca\01h\f1\fe\e1\8d\03x\ea\ca\8a\9e\04h\d6\a9\bb\ae\0ax\01\00\01\00\00\00\00\00\00\00\01",
)
# and it begins with "BTR", that means it is the begin of one btreemap.
```

dfx canister call balance stablesize



dfx canister call balance multiple '(0, 2000)'

dfx canister call balance page_info

dfx canister call balance multiple '(2000, 2000)'
 

 