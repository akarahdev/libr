#![feature(no_core)]

#![no_core]
#![no_std]

fn main() {
    libr::noop();
    unsafe { libr::libc::printf(c"hi gng\n".as_ptr()) };
}