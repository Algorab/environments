//! Special stuff for error handling

use std::panic;


 /// Custom silent catch_unwind. The normal catch_unwind prints the error, although the error case
 /// is handled.
pub fn catch_unwind_silent<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) -> std::thread::Result<R> {
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(f);
    panic::set_hook(prev_hook);
    result
}