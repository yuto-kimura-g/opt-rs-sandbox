# opt-rs-sandbox

Mathematical Optimization with Rust: sandbox

## Software

### Modeler
- `good_lp`
  - GitHub: https://github.com/rust-or/good_lp
  - docs: https://docs.rs/good_lp/latest/good_lp/index.html

### Solver
- `COIN-OR/Cbc`
  - GitHub: https://github.com/coin-or/Cbc
- `HiGHS`
  - GitHub: https://github.com/ERGO-Code/HiGHS
  - docs: https://highs.dev/

## Verify
```console
$ docker-compose up -d
$ docker exec -it opt-rs-sandbox bash
# cargo run --bin main
# exit
$ docker-compose down
$ docker ps -a
```
