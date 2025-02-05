#![allow(internal_features)]
#![feature(no_core, lang_items)]

#![no_core]
#![no_std]

pub mod libc;
pub mod rt;
pub mod marker;

pub fn noop() {}