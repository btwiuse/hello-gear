#![no_std]

#[no_mangle]
extern "C" fn handle() {
    let _ = gstd::msg::load_bytes(); // read input message and do nothing
    gstd::msg::reply_bytes(gstd::String::from("Hello world!"), 0).unwrap();
}

#[cfg(test)]
use gtest::{Program, System};

#[cfg(test)]
pub fn init_program(_prog: &Program) {
    // skip init
}

#[test]
fn it_works() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);
    init_program(&program);

    let res = program.send_bytes(42, "foo");
    assert_eq!(res.main_failed(), false);

    let res = program.send_bytes(69, "bar");
    assert_eq!(res.main_failed(), false);

    let res = program.send_bytes(1337, "baz");
    assert_eq!(res.main_failed(), false);
}
