#![warn(missing_docs)]

/*!
This crate provides a function, [`static_str()`], which provides
a static reference to a [`str`] by leaking it.

However, calling [`static_str()`] multiple times on the same string
(in the same thread) will return the same string every time to save memory.
*/

#[cfg(feature = "serde")]
mod static_str;
#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
pub use static_str::*;

use std::{cell::RefCell, collections::HashSet};

thread_local! {
    static STATIC_STRS: RefCell<HashSet<&'static str>> = Default::default();
}

/// Get a static reference to a `str`
///
/// This works by keeping a set of static leaked `str`s.
/// Calling this function with the same input `str` (and in the same thread) will
/// return the same static reference every time.
///
/// The amount of leaked memory scales with the number of
/// unique `str`s passed to this function, so only use it
/// on `str`s that you know there won't be too many of.
pub fn static_str(s: &str) -> &'static str {
    STATIC_STRS.with(|strs| {
        let mut strs = strs.borrow_mut();
        if !strs.contains(s) {
            strs.insert(Box::leak(s.to_string().into_boxed_str()));
        }
        *strs.get(s).unwrap()
    })
}

#[cfg(test)]
#[test]
fn it_works() {
    let a = static_str("a");
    let b = static_str("b");
    let c = static_str("c");
    let d = static_str("d");
    assert_eq!(a, "a");
    assert_eq!(b, "b");
    assert_eq!(c, "c");
    assert_eq!(d, "d");
}
