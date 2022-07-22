# balance
terminal 0:
```sh
dfx start --clean
```

terminal 1:
```sh
dfx deploy --no-wallet

dfx canister call balance stablesize
(0 : nat64)

Canister status call result for balance.
Status: Running
Controllers: yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae
Memory allocation: 0
Compute allocation: 0
Freezing threshold: 2_592_000
Memory Size: Nat(1753917)
Balance: 100_000_000_000_000 Cycles
Module hash: 0x48aa33fe2e780957cf08136a21d0d3349ab59ebbda083c47b59f4844fddae025

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
      timestamp = 1_658_474_215_242_670_000 : nat64;
    };
    record {
      id = 1 : nat64;
      to = principal "ktfx3-4dj7o-f4lqf-gab56-fgkuw-aagt6-jzpkd-o7xzp-f6a3p-nm6wl-wae";
      value = 2_000 : nat64;
      from = principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      timestamp = 1_658_474_825_329_986_000 : nat64;
    };
    record {
      id = 2 : nat64;
      to = principal "yd5hv-nayum-igkpt-jtrhr-aqfqq-pfkxn-fyxvh-yryn3-rez4o-p6vks-hqe";
      value = 3_000 : nat64;
      from = principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      timestamp = 1_658_474_829_511_266_000 : nat64;
    };
    record {
      id = 3 : nat64;
      to = principal "qbyxf-lll3o-rmkt4-thocb-vo3bz-d2mfj-3nwa7-pv2rf-iqfqw-rgkg2-hqe";
      value = 4_000 : nat64;
      from = principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      timestamp = 1_658_474_832_270_218_000 : nat64;
    };
  },
)
```

test upgrade:
```sh
# modify the src code to change the module hash to upgrade

dfx deploy --no-wallet

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
      timestamp = 1_658_474_215_242_670_000 : nat64;
    };
    record {
      id = 1 : nat64;
      to = principal "ktfx3-4dj7o-f4lqf-gab56-fgkuw-aagt6-jzpkd-o7xzp-f6a3p-nm6wl-wae";
      value = 2_000 : nat64;
      from = principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      timestamp = 1_658_474_825_329_986_000 : nat64;
    };
    record {
      id = 2 : nat64;
      to = principal "yd5hv-nayum-igkpt-jtrhr-aqfqq-pfkxn-fyxvh-yryn3-rez4o-p6vks-hqe";
      value = 3_000 : nat64;
      from = principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      timestamp = 1_658_474_829_511_266_000 : nat64;
    };
    record {
      id = 3 : nat64;
      to = principal "qbyxf-lll3o-rmkt4-thocb-vo3bz-d2mfj-3nwa7-pv2rf-iqfqw-rgkg2-hqe";
      value = 4_000 : nat64;
      from = principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      timestamp = 1_658_474_832_270_218_000 : nat64;
    };
  },
)

# Success
```

test large size:
```sh
dfx canister call balance stablesize
(225 : nat64)

dfx canister call balance page_info
(
  1 : nat64,
  vec { 0 : nat32 },
  123 : nat64,
  vec { 1 : nat32; 2 : nat32; 3 : nat32; 4 : nat32; 5 : nat32; 6 : nat32; 7 : nat32; 8 : nat32; 9 : nat32; 10 : nat32; 11 : nat32; 12 : nat32; 13 : nat32; 14 : nat32; 15 : nat32; 16 : nat32; 17 : nat32; 18 : nat32; 19 : nat32; 20 : nat32; 21 : nat32; 22 : nat32; 23 : nat32; 24 : nat32; 25 : nat32; 26 : nat32; 27 : nat32; 28 : nat32; 29 : nat32; 30 : nat32; 31 : nat32; 32 : nat32; 33 : nat32; 34 : nat32; 35 : nat32; 36 : nat32; 37 : nat32; 38 : nat32; 39 : nat32; 40 : nat32; 41 : nat32; 42 : nat32; 43 : nat32; 44 : nat32; 45 : nat32; 46 : nat32; 47 : nat32; 48 : nat32; 49 : nat32; 50 : nat32; 51 : nat32; 52 : nat32; 53 : nat32; 54 : nat32; 55 : nat32; 56 : nat32; 57 : nat32; 58 : nat32; 59 : nat32; 60 : nat32; 61 : nat32; 62 : nat32; 63 : nat32; 64 : nat32; 65 : nat32; 66 : nat32; 67 : nat32; 68 : nat32; 69 : nat32; 70 : nat32; 71 : nat32; 72 : nat32; 73 : nat32; 74 : nat32; 75 : nat32; 76 : nat32; 77 : nat32; 78 : nat32; 79 : nat32; 80 : nat32; 81 : nat32; 82 : nat32; 83 : nat32; 84 : nat32; 85 : nat32; 86 : nat32; 87 : nat32; 88 : nat32; 89 : nat32; 90 : nat32; 91 : nat32; 92 : nat32; 93 : nat32; 94 : nat32; 95 : nat32; 96 : nat32; 97 : nat32; 98 : nat32; 99 : nat32; 100 : nat32; 101 : nat32; 102 : nat32; 103 : nat32; 104 : nat32; 105 : nat32; 106 : nat32; 107 : nat32; 108 : nat32; 109 : nat32; 110 : nat32; 111 : nat32; 112 : nat32; 113 : nat32; 114 : nat32; 115 : nat32; 116 : nat32; 117 : nat32; 118 : nat32; 119 : nat32; 120 : nat32; 121 : nat32; 122 : nat32; 123 : nat32;},
)
# Page [0, 101) belongs to the Vistual Memory Manager. 
# Page [101, 102) belongs to the Balance. 
# Page [102, 225) belongs to the History.
# The stable Log is designed to store logs of different sizes so it needs to store the start address of each entry.
# Here [102, 224) is reserved for storing the every entry address.
#
# In Vistual Memory Manger's Record, the page 0 of RestrictedMemory [101..131072) belongs to balance, and its virtual memory's index = 0:
# [0, 0, 0, 0]
#  |  
# balance index
#
# the page 1 belongs to history, and its struct's index = 1:
# [1, 0, 0, 1]
#  | 
# history index
# so we can find them in \01\00\00\01\ below, also, \01\00\00\02\ can be found:
dfx canister call balance read_raw_memory '(0, 300)'
(
  blob "BTR\01\04\00\00\00\00\00\00\003\0d\00\00\00\00\00\00|\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00BTA\01\00\00\00\00\eb\00\00\00\00\00\00\00\18\00\00\00\00\00\00\00\ec\17\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00CHK\01\01\00\00\00\00\00\00\00\00\00\00\00BTN\01\00\05\00\04\00\00\00\00\00\00\00\00\00\00\00\04\00\00\00\01\00\00\01\00\00\00\00\04\00\00\00\01\00\00\02\00\00\00\00\04\00\00\00\01\00\00\03\00\00\00\00\04\00\00\00\01\00\00\04\00\00\00\00\04\00\00\00\01\00\00\05\00\00\00\00\04\00\00\00\01\00\00\06\00\00\00\00\04\00\00\00\01\00\00\07\00\00\00\00\04\00\00\00\01\00\00\08\00\00\00\00\04\00\00\00\01\00\00\09\00\00\00\00\04\00\00\00\01\00\00\0a\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00",
)
# and it begins with "BTR", that means it is the begin of one btreemap.

# get the balance raw:
# 101 * 65536 = 6_619_136
dfx canister call balance read_raw_memory '(6_619_136, 300)'
(
  blob "BTR\01\1d\00\00\00\08\00\00\00t\00\00\00\00\00\00\00\05\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00BTA\01\00\00\00\00V\02\00\00\00\00\00\00\01\00\00\00\00\00\00\00\ca\02\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00CHK\01\01\00\00\00\00\00\00\00\00\00\00\00BTN\01\00\05\00\00\00\00\00\98\ef\18\ac\8c\0c\d7\8e,)a\d8\f3\9e-\1a\c2\a0\a8\few\19\ef\82\d7B\d1\a6\02\08\00\00\00\00\00\00\00\00\00\03\e8\1d\00\00\00\18\a3\10e>i\9cO\10@\b0\83\ca\ab\b4\b8\bdO\88\e1\bb\893\c7?\d5T\8f\02\08\00\00\00\00\00\00\00\00\00\0b\b8\1d\00\00\00i\fb\8b\c5\c0\a6\00{\e2\99T\b0\00i\f99z\86\ef\df//\81\b7\b5\9e\b2\ec\02\08\00\00\00\00\00\00\00\00\00\07\d0\1d\00\00\00k\db\a2\c5O\93;\84\1a\bba\c8\f4\c2\a7m\b0>\fa\ea%D\0b\0bD\ca6\8f\02\08\00\00\00\00\00\00\00\00",
)
# and it begins with "BTR", that means it is the begin of one btreemap.

# get the history raw:
# 102 * 65536 = 6_684_672
dfx canister call balance read_raw_memory '(6_684_672, 300)'
(
  blob "SLG\01@B\0f\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\04\00\00\00\5c\00\00\00\00\00\00\00\d5\00\00\00\00\00\00\00N\01\00\00\00\00\00\00\c7\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00",
)
# and it begins with "SLG", that means it is the begin of one stable log.
```

Test multiple pages:
```sh
dfx canister call balance multiple '(0, 1000)'
(variant { Ok })

dfx canister call balance stablesize
(227 : nat64)

dfx canister call balance page_info
(
  2 : nat64,
  vec { 0 : nat32; 124 : nat32 },
  124 : nat64,
  vec { 1 : nat32; 2 : nat32; 3 : nat32; 4 : nat32; 5 : nat32; 6 : nat32; 7 : nat32; 8 : nat32; 9 : nat32; 10 : nat32; 11 : nat32; 12 : nat32; 13 : nat32; 14 : nat32; 15 : nat32; 16 : nat32; 17 : nat32; 18 : nat32; 19 : nat32; 20 : nat32; 21 : nat32; 22 : nat32; 23 : nat32; 24 : nat32; 25 : nat32; 26 : nat32; 27 : nat32; 28 : nat32; 29 : nat32; 30 : nat32; 31 : nat32; 32 : nat32; 33 : nat32; 34 : nat32; 35 : nat32; 36 : nat32; 37 : nat32; 38 : nat32; 39 : nat32; 40 : nat32; 41 : nat32; 42 : nat32; 43 : nat32; 44 : nat32; 45 : nat32; 46 : nat32; 47 : nat32; 48 : nat32; 49 : nat32; 50 : nat32; 51 : nat32; 52 : nat32; 53 : nat32; 54 : nat32; 55 : nat32; 56 : nat32; 57 : nat32; 58 : nat32; 59 : nat32; 60 : nat32; 61 : nat32; 62 : nat32; 63 : nat32; 64 : nat32; 65 : nat32; 66 : nat32; 67 : nat32; 68 : nat32; 69 : nat32; 70 : nat32; 71 : nat32; 72 : nat32; 73 : nat32; 74 : nat32; 75 : nat32; 76 : nat32; 77 : nat32; 78 : nat32; 79 : nat32; 80 : nat32; 81 : nat32; 82 : nat32; 83 : nat32; 84 : nat32; 85 : nat32; 86 : nat32; 87 : nat32; 88 : nat32; 89 : nat32; 90 : nat32; 91 : nat32; 92 : nat32; 93 : nat32; 94 : nat32; 95 : nat32; 96 : nat32; 97 : nat32; 98 : nat32; 99 : nat32; 100 : nat32; 101 : nat32; 102 : nat32; 103 : nat32; 104 : nat32; 105 : nat32; 106 : nat32; 107 : nat32; 108 : nat32; 109 : nat32; 110 : nat32; 111 : nat32; 112 : nat32; 113 : nat32; 114 : nat32; 115 : nat32; 116 : nat32; 117 : nat32; 118 : nat32; 119 : nat32; 120 : nat32; 121 : nat32; 122 : nat32; 123 : nat32; 125 : nat32;},
)

dfx canister call balance multiple '(1000, 1000)'
(variant { Ok })

dfx canister call balance page_info
(
  4 : nat64,
  vec { 0 : nat32; 124 : nat32; 126 : nat32; 127 : nat32 },
  125 : nat64,
  vec { 1 : nat32; 2 : nat32; 3 : nat32; 4 : nat32; 5 : nat32; 6 : nat32; 7 : nat32; 8 : nat32; 9 : nat32; 10 : nat32; 11 : nat32; 12 : nat32; 13 : nat32; 14 : nat32; 15 : nat32; 16 : nat32; 17 : nat32; 18 : nat32; 19 : nat32; 20 : nat32; 21 : nat32; 22 : nat32; 23 : nat32; 24 : nat32; 25 : nat32; 26 : nat32; 27 : nat32; 28 : nat32; 29 : nat32; 30 : nat32; 31 : nat32; 32 : nat32; 33 : nat32; 34 : nat32; 35 : nat32; 36 : nat32; 37 : nat32; 38 : nat32; 39 : nat32; 40 : nat32; 41 : nat32; 42 : nat32; 43 : nat32; 44 : nat32; 45 : nat32; 46 : nat32; 47 : nat32; 48 : nat32; 49 : nat32; 50 : nat32; 51 : nat32; 52 : nat32; 53 : nat32; 54 : nat32; 55 : nat32; 56 : nat32; 57 : nat32; 58 : nat32; 59 : nat32; 60 : nat32; 61 : nat32; 62 : nat32; 63 : nat32; 64 : nat32; 65 : nat32; 66 : nat32; 67 : nat32; 68 : nat32; 69 : nat32; 70 : nat32; 71 : nat32; 72 : nat32; 73 : nat32; 74 : nat32; 75 : nat32; 76 : nat32; 77 : nat32; 78 : nat32; 79 : nat32; 80 : nat32; 81 : nat32; 82 : nat32; 83 : nat32; 84 : nat32; 85 : nat32; 86 : nat32; 87 : nat32; 88 : nat32; 89 : nat32; 90 : nat32; 91 : nat32; 92 : nat32; 93 : nat32; 94 : nat32; 95 : nat32; 96 : nat32; 97 : nat32; 98 : nat32; 99 : nat32; 100 : nat32; 101 : nat32; 102 : nat32; 103 : nat32; 104 : nat32; 105 : nat32; 106 : nat32; 107 : nat32; 108 : nat32; 109 : nat32; 110 : nat32; 111 : nat32; 112 : nat32; 113 : nat32; 114 : nat32; 115 : nat32; 116 : nat32; 117 : nat32; 118 : nat32; 119 : nat32; 120 : nat32; 121 : nat32; 122 : nat32; 123 : nat32; 125 : nat32; 128 : nat32;},
)

dfx canister call balance get_history '(0, 3100)'
...
```
