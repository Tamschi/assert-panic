/// Panics if `$stmt` doesn't panic.  
/// Optionally asserts the type of the panic.  
/// Optionally asserts a panic text start, or a given panic value.
///
/// # Example
///
/// ```rust
/// # use std::any::Any;
/// use assert_panic::assert_panic;
///
/// let _: Box<dyn Any + Send + 'static> = assert_panic!(panic!("at the Disco"));
/// let _: () = assert_panic!(panic!("at the Disco"), &str);
/// let _: () = assert_panic!({ assert_panic!({}); }, String, starts with "assert_panic! argument did not panic:");
/// let _: () = assert_panic!(
///     assert_panic!(panic!("found"), &str, starts with "expected"),
///     String,
///     "Expected a panic starting with \"expected\" but found \"found\"");
/// let _: () = assert_panic!(
///     assert_panic!(panic!(1usize), usize, 2usize),
///     String,
///     "Expected 2 but found 1"
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
                "Expected a `{}` panic but found something with TypeId {:?}",
                stringify!($ty),
                panic.type_id()
            )
        });
    }};

    ($stmt:stmt, $ty:ty, starts with $expr:expr$(,)?) => {{
        let panic = $crate::assert_panic!($stmt);
        let panic = panic.downcast_ref::<$ty>().unwrap_or_else(|| {
            panic!(
                "Expected a `{}` panic but found something with TypeId {:?}",
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
                "Expected a `{}` panic but found something with TypeId {:?}",
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

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn no_panic() {
        use std::any::Any;
        let _: Box<dyn Any> = assert_panic!({});
    }

    #[test]
    #[should_panic]
    #[allow(clippy::let_unit_value)]
    fn wrong_text() {
        let _: () = assert_panic!(panic!("at the Disco"), &str, "at");
    }

    #[test]
    #[should_panic]
    #[allow(clippy::let_unit_value)]
    fn wrong_start() {
        let _: () = assert_panic!(panic!("at the Disco"), &str, starts with "aaa");
    }
}
