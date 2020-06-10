//! Assert that a panic happens, and optionally what (kind of) panic happens.

#![doc(html_root_url = "https://docs.rs/assert-panic/1.0.0")]
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
///
/// Optionally asserts the downcast panic `contains`, `starts with` or equals a given expression `$expr`.
///
/// # Panics
///
/// - if `$stmt` doesn't panic.
/// - optionally if the type of the panic doesn't match.
/// - optionally if the panic has the wrong value.
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
///     starts with "Expected a `String` panic but found one with TypeId { t: ",
/// );
///
/// # let _: () =
/// assert_panic!(
///     assert_panic!(panic!("found"), &str, contains "expected"),
///     String,
///     "Expected a panic containing \"expected\" but found \"found\"",
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
///     assert_panic!(panic!(1_usize), usize, 2_usize),
///     String,
///     "Expected a panic equal to 2 but found 1",
/// );
/// ```
/// 
/// # Details
///
/// All arguments are evaluated at most once, but `$expr` must be [`Copy`](https://doc.rust-lang.org/stable/std/marker/trait.Copy.html).
///
/// `$expr` is only evaluated if `$stmt` panics.
/// 
/// Type assertions use [`Any::downcast_ref::<$ty>()`](https://doc.rust-lang.org/stable/std/any/trait.Any.html#method.downcast_ref-1).
///
/// The value is examined by reference `panic` only after downcasting it to `$ty`:
///
/// - `contains` uses `panic.contains(expr)`.  
/// - `starts with` uses `panic.starts_with(expr)`.  
/// - Equality comparison is done with `*panic == expr`.
///
/// All of this is duck-typed, so the respective forms on require that matching methods / the matching operator are present.
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
                "Expected a `{}` panic but found one with {:?}",
                stringify!($ty),
                panic.type_id()
            )
        });
    }};

    ($stmt:stmt, $ty:ty, contains $expr:expr$(,)?) => {{
        let panic = $crate::assert_panic!($stmt);
        let expr = $expr;
        let panic = panic.downcast_ref::<$ty>().unwrap_or_else(|| {
            panic!(
                "Expected a `{}` panic containing {:?} but found one with {:?}",
                stringify!($ty),
                expr,
                panic.type_id()
            )
        });
        assert!(
            panic.contains(expr),
            "Expected a panic containing {:?} but found {:?}",
            expr,
            panic
        );
    }};

    ($stmt:stmt, $ty:ty, starts with $expr:expr$(,)?) => {{
        let panic = $crate::assert_panic!($stmt);
        let expr = $expr;
        let panic = panic.downcast_ref::<$ty>().unwrap_or_else(|| {
            panic!(
                "Expected a `{}` panic starting with {:?} but found one with {:?}",
                stringify!($ty),
                expr,
                panic.type_id()
            )
        });
        assert!(
            panic.starts_with(expr),
            "Expected a panic starting with {:?} but found {:?}",
            expr,
            panic
        );
    }};

    ($stmt:stmt, $ty:ty, $expr:expr$(,)?) => {{
        let panic = $crate::assert_panic!($stmt);
        let expr = $expr;
        let panic = panic.downcast_ref::<$ty>().unwrap_or_else(|| {
            panic!(
                "Expected a `{}` panic equal to {:?} but found one with {:?}",
                stringify!($ty),
                expr,
                panic.type_id()
            )
        });
        assert!(*panic == expr, "Expected a panic equal to {:?} but found {:?}", expr, panic);
    }};
}
