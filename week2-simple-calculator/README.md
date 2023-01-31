# Mini Project week2

## 0. Dependencies and Preparation

install lambda first `pip install cargo-lambda`

## 1. Usage

`cd week2-simple-calculator`

To deploy: `make deploy` which runs: `cargo lambda build --release`

Test locally with remote invoke on AWS lambda: `make invoke`

```bash
cargo lambda invoke --remote \
        --data-ascii '{"expression": "1+2*5-(12/4+7*(1+2))"}' \
        --output-format json \
        week2
```

Result:
```bash
cargo lambda invoke --remote \
        --data-ascii '{"expression": "1+2*5-(12/4+7*(1+2))"}' \
        --output-format json \
        week2
{
  "result": "-13"
}
```
