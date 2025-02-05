#![allow(internal_features)]
#![feature(no_core, lang_items)]

#![no_core]
#![no_std]

#[cfg(target_os = "linux")]
#[link(name = "c")]
unsafe extern "C" {}

pub mod rt;
pub mod marker;

pub fn noop() {}