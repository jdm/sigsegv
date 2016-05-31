#![feature(core_intrinsics)]

extern crate backtrace;
extern crate simple_signal;
use backtrace::Backtrace;
use simple_signal::{Signals, Signal};
use std::intrinsics::abort;
use std::ptr;

#[inline(never)]
fn foo() {
    let p: *mut u32 = ptr::null_mut();
    unsafe { *p = 0 };
    println!("hey there");
}

fn main() {
    Signals::set_handler(&[Signal::Segv], move |_signals| {
        println!("Backtrace\n{:?}", Backtrace::new());
        unsafe { abort(); }
    });
    println!("hi");
    foo();
    println!("bye");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
