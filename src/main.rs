extern crate nix;

use nix::unistd::execv;

use std::ffi::CStr;

fn main() {
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
