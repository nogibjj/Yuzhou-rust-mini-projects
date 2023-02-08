# Mini Project week3

## 0. Dependencies and Preparation

First, install lambda in your venv `pip install cargo-lambda`

## 1. Usage

`cd week2-simple-calculator`

To deploy: `make deploy` which runs: `cargo lambda build --release`

Test locally with remote invoke on AWS lambda: `make invoke`

```bash
cargo lambda invoke --remote \
        --data-ascii '{"integer": "112"}' \
        --output-format json \
        week3
```

Result:
```bash
cargo lambda invoke --remote \
        --data-ascii '{"integer": "112"}' \
        --output-format json \
        week3
{
  "result": "CXII"
}
```
