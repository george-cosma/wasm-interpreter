pub mod common;

/// A simple function to add 1 to an i32 and return the result
#[test_log::test]
fn add_one() {
    use wasm::{validate, RuntimeInstance};

    let wat = r#"
    (module
        (func (export "add_one") (param $x i32) (result i32)
            local.get $x
            i32.const 1
            i32.add)
    )
    "#;
    test_init!(wat, (wasmtime, instance, wasmi));

    // .---------------------------------------------------------------.
    // | Variant 1, more control over expected type and the assertions |
    // '---------------------------------------------------------------'
    let mut r1: i32;
    let mut r2: i32;
    let mut r3: i32;
    test_run!(
        (0, "add_one"),
        11,
        (r1, r2, r3),
        (wasmtime, instance, wasmi)
    );
    assert_eq!(12, r1);
    assert_eq!(12, r2);
    assert_eq!(12, r3);
    // or other, more complicated, checks
    test_run!((0, "add_one"), 0, (r1, r2, r3), (wasmtime, instance, wasmi));
    assert_eq!(1, r1);
    assert_eq!(1, r2);
    assert_eq!(1, r3);
    // or other, more complicated, checks

    // .-----------.
    // | Variant 2 |
    // '-----------'
    test_eq!((0, "add_one"), 11, (12, i32), (wasmtime, instance, wasmi));
    test_eq!((0, "add_one"), 0, (1, i32), (wasmtime, instance, wasmi));
    test_eq!((0, "add_one"), -5, (-6, i32), (wasmtime, instance, wasmi));
}
