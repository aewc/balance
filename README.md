# balance
terminal 1:
```sh
dfx start --clean
```

terminal 2:
```sh
dfx deploy
```

terminal 3:
```sh
cargo bench
   Compiling balance v0.1.0 (/Users/flyq/workspace/github-aewc/balance/src/balance)
    Finished bench [optimized] target(s) in 5.52s
     Running unittests src/main.rs (target/release/deps/balance-dd6b64458ddd3e6e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/my_benchmark.rs (target/release/deps/my_benchmark-5cd578e4bf50f1c9)
Gnuplot not found, using plotters backend
Benchmarking write_map_stable 0-100_000: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 66.3s.
write_map_stable 0-100_000
                        time:   [5.1605 s 5.3159 s 5.4723 s]
                        change: [-4.7609% -0.9338% +3.1013%] (p = 0.74 > 0.05)
                        No change in performance detected.

Benchmarking write_map_stable 100_000-200_000: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 76.6s.
write_map_stable 100_000-200_000
                        time:   [5.3128 s 5.4703 s 5.6241 s]
                        change: [-1.9181% +1.9482% +5.9758%] (p = 0.38 > 0.05)
                        No change in performance detected.

Benchmarking write_map_wasm_heap 0-100_000: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 41.0s.
write_map_wasm_heap 0-100_000
                        time:   [3.1301 s 3.2833 s 3.4370 s]
                        change: [-6.1121% +0.0633% +6.6569%] (p = 0.92 > 0.05)
                        No change in performance detected.

Benchmarking write_map_wasm_heap 100_000-200_000: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 46.0s.
write_map_wasm_heap 100_000-200_000
                        time:   [3.1272 s 3.2818 s 3.4372 s]
                        change: [-7.9418% -1.6977% +4.9765%] (p = 0.59 > 0.05)
                        No change in performance detected.

Benchmarking write_vec_stable 0-100_000_000: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 30.9s.
write_vec_stable 0-100_000_000
                        time:   [2.6187 s 3.0397 s 3.6775 s]
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe

Benchmarking write_vec_stable 100_000_000-200_000_000: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 30.8s.
write_vec_stable 100_000_000-200_000_000
                        time:   [2.5713 s 2.6845 s 2.8269 s]
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high severe

Benchmarking write_vec_wasm_heap 0-100_000_000: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 25.6s.
write_vec_wasm_heap 0-100_000_000
                        time:   [2.9274 s 3.2948 s 3.8765 s]
Found 3 outliers among 10 measurements (30.00%)
  1 (10.00%) low severe
  1 (10.00%) high mild
  1 (10.00%) high severe

Benchmarking write_vec_wasm_heap 100_000_000-2_000_000_000: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 46.1s.
write_vec_wasm_heap 100_000_000-2_000_000_000
                        time:   [2.6730 s 2.8261 s 2.9782 s]

Benchmarking read_map_stable 0-100_000: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 33.9s.
read_map_stable 0-100_000
                        time:   [3.3683 s 3.6827 s 4.1565 s]
                        change: [-1.4091% +7.8879% +21.679%] (p = 0.30 > 0.05)
                        No change in performance detected.
Found 4 outliers among 10 measurements (40.00%)
  1 (10.00%) low severe
  1 (10.00%) low mild
  2 (20.00%) high severe

Benchmarking read_map_stable 100_000-200_000: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 35.5s.
read_map_stable 100_000-200_000
                        time:   [3.4906 s 3.4959 s 3.5012 s]
                        change: [-24.712% -12.376% -0.6722%] (p = 0.07 > 0.05)
                        No change in performance detected.

Benchmarking read_map_wasm_heap 0-100_000: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 36.8s.
read_map_wasm_heap 0-100_000
                        time:   [1.7549 s 1.9010 s 2.1909 s]
                        change: [-1.2953% +6.9612% +23.338%] (p = 0.63 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe

Benchmarking read_map_wasm_heap 100_000-200_000: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 18.1s.
read_map_wasm_heap 100_000-200_000
                        time:   [1.8187 s 1.8234 s 1.8296 s]
                        change: [-42.519% -22.037% -0.9328%] (p = 0.18 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild

Benchmarking read_vec_stable 0-100_000_000: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 12.6s.
read_vec_stable 0-100_000_000
                        time:   [1.1862 s 1.1934 s 1.2012 s]

Benchmarking read_vec_stable 100_000_000-200_000_000: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 12.0s.
read_vec_stable 100_000_000-200_000_000
                        time:   [1.1972 s 1.5812 s 2.3283 s]
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe

Benchmarking read_vec_wasm_heap 0-100_000_000: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 18.9s.
read_vec_wasm_heap 0-100_000_000
                        time:   [1.8044 s 1.8092 s 1.8137 s]
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) low mild

Benchmarking read_vec_wasm_heap 100_000_000-200_000_000: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 18.4s.
read_vec_wasm_heap 100_000_000-200_000_000
                        time:   [1.8087 s 1.8179 s 1.8289 s]

```