//! Assert that a panic happens, and optionally what (kind of) panic happens.

#![doc(html_root_url = "https://docs.rs/assert-panic/1.0.0-preview.1")]
#![doc(test(no_crate_inject))]
#![warn(
    clippy::as_conversions,
    clippy::cargo,
    clippy::clone_on_ref_ptr,
    clippy::missing_docs_in_private_items,
    clippy::pedantic
)]
// Debug cleanup. Uncomment before committing.
#![forbid(
    clippy::dbg_macro,
    clippy::print_stdout,
    clippy::todo,
    clippy::unimplemented
)]

/// Asserts that `$stmt` panics.  
///
/// - Only this base form with a single expression returns the panic.
///
/// Optionally asserts the type of the panic.  
/// Optionally asserts a panic text start, or a given panic value.
///
/// # Panics
///
/// - if `$stmt` doesn't panic.
/// - optionally if the type of the panic doesn't match.
/// - optionally if the panic text starts in the wrong way, or the panic has the wrong value.
///
/// # Example
///
/// ```rust
/// # use std::any::Any;
/// use assert_panic::assert_panic;
///
/// let _: Box<dyn Any + Send + 'static> =
///     assert_panic!(panic!("at the Disco"));
///
/// # let _: () =
/// assert_panic!(panic!("at the Disco"), &str);
///
/// # let _: () =
/// assert_panic!(
///     { assert_panic!({}); },
///     String,
///     starts with "assert_panic! argument did not panic:",
/// );
///
/// # let _: () =
/// assert_panic!(
///     assert_panic!(panic!("at the Disco"), String),
///     String,
///     starts with "Expected a `String` panic but found something with TypeId { t: ",
/// );
///
/// # let _: () =
/// assert_panic!(
///     assert_panic!(panic!("found"), &str, starts with "expected"),
///     String,
///     "Expected a panic starting with \"expected\" but found \"found\"",
/// );
///
/// # let _: () =
/// assert_panic!(
///     assert_panic!(panic!(1usize), usize, 2usize),
///     String,
///     "Expected 2 but found 1",
/// );
/// ```
#[macro_export]
macro_rules! assert_panic {
    ($stmt:stmt$(,)?) => {
        ::std::panic::catch_unwind(|| -> () { $stmt })
            .expect_err("assert_panic! argument did not panic")
    };

    ($stmt:stmt, $ty:ty$(,)?) => {{
        let panic = $crate::assert_panic!($stmt);
        panic.downcast_ref::<$ty>().unwrap_or_else(|| {
            panic!(
                "Expected a `{}` panic but found something with {:?}",
                stringify!($ty),
                panic.type_id()
            )
        });
    }};

    ($stmt:stmt, $ty:ty, starts with $expr:expr$(,)?) => {{
        let panic = $crate::assert_panic!($stmt);
        let panic = panic.downcast_ref::<$ty>().unwrap_or_else(|| {
            panic!(
                "Expected a `{}` panic but found something with {:?}",
                stringify!($ty),
                panic.type_id()
            )
        });
        assert!(
            panic.starts_with($expr),
            "Expected a panic starting with {:?} but found {:?}",
            $expr,
            panic
        );
    }};

    ($stmt:stmt, $ty:ty, $expr:expr$(,)?) => {{
        let panic = $crate::assert_panic!($stmt);
        let panic = panic.downcast_ref::<$ty>().unwrap_or_else(|| {
            panic!(
                "Expected a `{}` panic but found something with {:?}",
                stringify!($ty),
                panic.type_id()
            )
        });
        assert!(
            *panic == $expr,
            "Expected {:?} but found {:?}",
            $expr,
            panic
        );
    }};
}
