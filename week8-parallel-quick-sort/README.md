# Mini Project week8

## 1. Setup a Rust and Python interface

`cargo run -- --help` This will show what your subcommand will look like

`cargo run quick-sort` This will run quicksort on a vector of size 1000000 (default value)

`cargo run quick-sort --size <your size>` This will run quicksort on a vector of your defined size

## Example:
```
Sorting vector of size: 1000000
Nanosecond elapsed: 170686379
is sorted: true
Nanosecond elapsed: 598082535
is sorted: true
speedup: 3.503985136388651
```
```
Sorting vector of size: 10000000
Nanosecond elapsed: 1563949016
is sorted: true
Nanosecond elapsed: 6969250601
is sorted: true
speedup: 4.456187848645317
```