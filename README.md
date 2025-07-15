Tested only on ubuntu 24.04

### Prerequisites

Install all required packages from rust [riscv64-unknown-linux-gnu](https://doc.rust-lang.org/nightly/rustc/platform-support/riscv64gc-unknown-linux-gnu.html) spec including `qemu-system-riscv64` package.

### Run
```cargo --config config.toml run --target riscv64gc-unknown-linux-gnu --release```
