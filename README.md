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

## `cargo test`

```
$ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (target/debug/deps/hello_gear-1c94104e0f4ac509)

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

   Doc-tests hello-gear

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## References

- https://doc.rust-lang.org/book/ch11-00-testing.html
- https://wiki.gear-tech.io/docs/developing-contracts/testing
  - https://wiki.gear-tech.io/docs/developing-contracts/testing-gtest
  - https://wiki.gear-tech.io/docs/developing-contracts/testing-gclient
- https://oneblock-gear-workshop-2022.vercel.app/#5
- https://github.com/gear-dapps/app
- https://github.com/gear-dapps/hello-world
- https://docs.gear.rs/gtest/
  - https://docs.gear.rs/gtest/struct.System.html
  - https://docs.gear.rs/gtest/struct.Program.html
  - https://docs.gear.rs/gtest/struct.RunResult.html
- https://github.com/btwiuse/gm
- debug
  - https://docs.gear.rs/gstd/macro.debug.html
    - https://docs.gear.rs/src/gstd/macros/debug.rs.html#24-34
  - https://docs.gear.rs/gcore/ext/fn.debug.html
    - https://github.com/gear-tech/gear/blob/07f831d5806b42e32769ae2ec6d27e052161bc4c/gcore/src/utils.rs#L24
  - https://github.com/gear-tech/gear/blob/07f831d5806b42e32769ae2ec6d27e052161bc4c/utils/wasm-instrument/src/syscalls.rs#L113
    - https://github.com/gear-tech/gear/blob/07f831d5806b42e32769ae2ec6d27e052161bc4c/core/src/costs.rs#L134
