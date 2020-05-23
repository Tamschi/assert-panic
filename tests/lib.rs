use assert_panic::assert_panic;

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
