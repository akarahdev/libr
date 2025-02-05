#[lang = "sized"]
pub trait Sized {}

#[lang = "copy"]
pub trait Copy {}

#[lang = "freeze"]
pub trait Freeze {}

#[lang = "destruct"]
pub trait Destruct {}

#[lang = "legacy_receiver"]
#[doc(hidden)]
pub trait LegacyReceiver {
    // Empty.
}
impl<T: ?Sized> LegacyReceiver for &T {}
impl<T: ?Sized> LegacyReceiver for &mut T {}

#[lang = "termination"]
pub trait Termination {
    fn report(self) -> i32;
}

impl Termination for () {
    fn report(self) -> i32 {
        0
    }
}