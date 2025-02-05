#[lang = "sized"]
pub trait Sized {}

#[lang = "copy"]
pub trait Copy {}

#[lang = "freeze"]
pub trait Freeze {}

#[lang = "destruct"]
pub trait Destruct {}

#[lang = "termination"]
pub trait Termination {
    fn report(self) -> i32;
}

impl Termination for () {
    fn report(self) -> i32 {
        0
    }
}