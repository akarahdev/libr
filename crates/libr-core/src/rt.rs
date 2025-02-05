use crate::marker::Termination;

#[lang = "start"]
fn start<T: Termination + 'static>(
    main: fn() -> T,
    _argc: isize,
    _argv: *const *const u8,
    _sigpipe: u8
) -> isize {
    main().report() as isize
}