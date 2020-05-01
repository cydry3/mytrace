extern crate nix;

use nix::sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet, Signal};
use nix::unistd::execv;

use std::ffi::CStr;
use std::os::raw::c_int;

use std::env;
use std::thread::sleep;
use std::time::Duration;

pub extern "C" fn echo_hello_world(_: c_int) {
    let args = vec!["/bin/echo\0", "Hello, world!\0"];

    let argv: Vec<_> = args
        .into_iter()
        .map(|a| CStr::from_bytes_with_nul(a.as_bytes()))
        .filter_map(|a| a.ok())
        .collect();

    if !(argv.len() > 0) {
        panic!("parsing error; 0 command/argument.")
    }

    execv(argv[0], &argv[..]).expect("Error in execv.");
}

pub extern "C" fn print_message(_: c_int) {
    println!("Hello, signal handler!");
}

fn main() {
    let mut args = env::args();
    let _command = args.next();
    let sub_command = args.next().unwrap();

    let handler = match &*sub_command {
        "hand" => SigHandler::Handler(print_message),
        "echo" => SigHandler::Handler(echo_hello_world),
        _ => panic!("unexpected argument."),
    };

    let sa = SigAction::new(handler, SaFlags::empty(), SigSet::empty());

    unsafe {
        sigaction(Signal::SIGINT, &sa).expect("fail making action");
    }

    loop {
        sleep(Duration::from_secs(1));
    }
}
