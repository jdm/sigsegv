#![feature(core_intrinsics)]

extern crate backtrace;
#[macro_use]
extern crate sig;
use backtrace::Backtrace;
use sig::ffi::Sig;
use std::intrinsics::abort;
use std::ptr;

#[inline(never)]
fn foo() {
    let p: *mut u32 = ptr::null_mut();
    unsafe { *p = 0 };
    println!("hey there");
}

fn handler(sig: i32) {
    println!("Backtrace\n{:?}", Backtrace::new());
    unsafe { abort(); }
}

fn main() {
    signal!(Sig::SEGV, handler);
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
