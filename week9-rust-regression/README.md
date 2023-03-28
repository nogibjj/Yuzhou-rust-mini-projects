# Mini Project week9

## 1. Setup a Rust and Python interface

`cargo run -- --help` This will show what your subcommand will look like

`cargo run regression` This will run default examples
`cargo run regression --samples 1000 --learning-rate 0.01 --epochs 1000 --true-slope 2.0 --true-intercept 1.0` This will run quicksort on a vector of your defined size

## Example:
```
Running linear regression model
True slope: 2.00
Estimated slope: 1.99
True intercept: 10.00
Estimated intercept: 10.01
```
```
Running linear regression model
True slope: 2.00
Estimated slope: 1.68
True intercept: 1.00
Estimated intercept: 1.20
```