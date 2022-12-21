# Hello Gear

## `cargo new ...`

```
$ cargo new --lib hello-gear && cd hello-gear
```

## `cargo add ...`

```
$ cargo add --git https://github.com/gear-tech/gear.git --build gear-wasm-builder
$ cargo add --git https://github.com/gear-tech/gear.git --dev gtest
$ cargo add --git https://github.com/gear-tech/gear.git gstd
```

## `./rust-toolchain`

```
[toolchain]
channel = "nightly"
components = [ "rustfmt", "clippy" ]
targets = [ "wasm32-unknown-unknown" ]
profile = "minimal"
```

## `./src/lib.rs`

```
#![no_std]

#[no_mangle]
extern "C" fn handle() {
  let _ = gstd::msg::load_bytes(); // read input message and do nothing 
  gstd::msg::reply_bytes(gstd::String::from("Hello world!"), 0).unwrap();
}
```

## `./build.rs`

```
fn main() {
    gear_wasm_builder::build();
}
```

## `cargo build --release`

- `./target/wasm32-unknown-unknown/release/hello_gear.opt.wasm`
  - 合约代码 (Code)
  - 提交上链 -> CodeId -> 部署 (+salt) -> ProgramId
- `./target/wasm32-unknown-unknown/release/hello_gear.meta.wasm`
  - 合约 Metadata

