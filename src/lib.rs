#![no_std]

#[no_mangle]
extern "C" fn handle() {
    let msg_bytes = gstd::msg::load_bytes().unwrap(); // read input message bytes
    gstd::debug!("handle() is being called with bytes: {:?}", msg_bytes);
    gstd::msg::reply_bytes(gstd::String::from("Hello world!"), 0).unwrap();
}

#[no_mangle]
extern "C" fn init() {
    let msg_bytes = gstd::msg::load_bytes().unwrap(); // read input message bytes
    gstd::debug!("init() is being called with bytes: {:?}", msg_bytes);
}

#[cfg(test)]
use gtest::{Program, System};

#[cfg(test)]
pub fn init_program(prog: &Program) {
    // skip init
    prog.send_bytes(0, "init");
}

#[test]
fn it_works() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);
    assert_eq!(program.id(), 1.into());

    let program2 = Program::current(&system);
    assert_eq!(program2.id(), 2.into());

    let program3 = Program::current_with_id(&system, 3);
    assert_eq!(program3.id(), 3.into());

    let program4 = Program::current(&system);
    assert_eq!(program4.id(), 4.into());

    init_program(&program);

    let res = program.send_bytes(42, "foo");
    assert_eq!(res.main_failed(), false);

    let res = program.send_bytes(69, "bar");
    assert_eq!(res.main_failed(), false);

    let res = program.send_bytes(1337, "baz");
    assert_eq!(res.main_failed(), false);
}
