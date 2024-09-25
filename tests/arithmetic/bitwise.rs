use wasm::{validate, RuntimeInstance};

const BASE_WAT: &'static str = r#"
    (module
      (func (export "template") (param $x i32) (param $y i32) (result i32)
          local.get $x
          local.get $y
          i32.{{0}})
    )
"#;

const BASE_COUNT_WAT: &'static str = r#"
    (module
      (func (export "template") (param $x i32) (result i32)
          local.get $x
          i32.{{0}})
    )
"#;
/// A simple function to test the i32.and bitwise operation
#[test_log::test]
pub fn i32_bitwise_and() {
    let wat = String::from(BASE_WAT).replace("{{0}}", "and");

    let wasm_bytes = wat::parse_str(wat).unwrap();

    let validation_info = validate(&wasm_bytes).expect("validation failed");

    let mut instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    assert_eq!(
        1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (33, 11))
            .unwrap()
    );
    assert_eq!(
        5,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (77, 23))
            .unwrap()
    );
    assert_eq!(
        180244,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (192534, 1231412)
            )
            .unwrap()
    );
    assert_eq!(
        0,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MIN, i32::MAX)
            )
            .unwrap()
    );
}

/// A simple function to test the i32.or bitwise operation
#[test_log::test]
pub fn i32_bitwise_or() {
    let wat = String::from(BASE_WAT).replace("{{0}}", "or");

    let wasm_bytes = wat::parse_str(wat).unwrap();

    let validation_info = validate(&wasm_bytes).expect("validation failed");

    let mut instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    assert_eq!(
        43,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (33, 11))
            .unwrap()
    );
    assert_eq!(
        95,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (77, 23))
            .unwrap()
    );
    assert_eq!(
        1243702,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (192534, 1231412)
            )
            .unwrap()
    );
    assert_eq!(
        -1,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MIN, i32::MAX)
            )
            .unwrap()
    );
}

/// A simple function to test the i32.xor bitwise operation
#[test_log::test]
pub fn i32_bitwise_xor() {
    let wat = String::from(BASE_WAT).replace("{{0}}", "xor");

    let wasm_bytes = wat::parse_str(wat).unwrap();

    let validation_info = validate(&wasm_bytes).expect("validation failed");

    let mut instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    assert_eq!(
        42,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (33, 11))
            .unwrap()
    );
    assert_eq!(
        90,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (77, 23))
            .unwrap()
    );
    assert_eq!(
        1063458,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (192534, 1231412)
            )
            .unwrap()
    );
    assert_eq!(
        -1,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MIN, i32::MAX)
            )
            .unwrap()
    );
}

/// A simple function to test the i32.shl bitwise operation
#[test_log::test]
pub fn i32_bitwise_shl() {
    let wat = String::from(BASE_WAT).replace("{{0}}", "shl");

    let wasm_bytes = wat::parse_str(wat).unwrap();

    let validation_info = validate(&wasm_bytes).expect("validation failed");

    let mut instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    assert_eq!(
        67584,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (33, 11))
            .unwrap()
    );
    assert_eq!(
        645922816,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (77, 23))
            .unwrap()
    );
    assert_eq!(
        23068672,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (192534, 1231412)
            )
            .unwrap()
    );
    assert_eq!(
        0,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MIN, i32::MAX)
            )
            .unwrap()
    );
}

/// A simple function to test the i32.shr_s bitwise operation
#[test_log::test]
pub fn i32_bitwise_shr_s() {
    let wat = String::from(BASE_WAT).replace("{{0}}", "shr_s");

    let wasm_bytes = wat::parse_str(wat).unwrap();

    let validation_info = validate(&wasm_bytes).expect("validation failed");

    let mut instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    assert_eq!(
        8881445,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (142_103_123, 4)
            )
            .unwrap()
    );
    assert_eq!(
        23879,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (391_248_921, 14)
            )
            .unwrap()
    );
    assert_eq!(
        601955006,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (1_203_910_012, 33)
            )
            .unwrap()
    );
    assert_eq!(
        1056594615,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (2_113_189_231, 33)
            )
            .unwrap()
    );
    assert_eq!(
        -1,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MIN, i32::MAX)
            )
            .unwrap()
    );

    // Basic positive number
    assert_eq!(
        4,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (8, 1))
            .unwrap()
    );

    // Shifting by 0 (no shift)
    assert_eq!(
        -1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-1, 0))
            .unwrap()
    );
    assert_eq!(
        1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (1, 0))
            .unwrap()
    );

    // Shifting negative numbers
    assert_eq!(
        -4,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-8, 1))
            .unwrap()
    );
    assert_eq!(
        -1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-1, 1))
            .unwrap()
    );

    // Shifting by 31 (maximum shift for 32-bit int)
    assert_eq!(
        -1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-1, 31))
            .unwrap()
    );
    assert_eq!(
        -1,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MIN, 31)
            )
            .unwrap()
    );
    assert_eq!(
        0,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MAX, 31)
            )
            .unwrap()
    );

    // Shifting by more than 31
    assert_eq!(
        -1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-1, 32))
            .unwrap()
    );
    assert_eq!(
        1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (1, 32))
            .unwrap()
    );
    assert_eq!(
        -1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-1, 100))
            .unwrap()
    );

    // Minimum and maximum 32-bit integers
    assert_eq!(
        i32::MIN / 2,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MIN, 1)
            )
            .unwrap()
    );
    assert_eq!(
        i32::MAX / 2,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MAX, 1)
            )
            .unwrap()
    );

    // Shifting out all bits except sign
    assert_eq!(
        -2,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MIN, 30)
            )
            .unwrap()
    );
    assert_eq!(
        1,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MAX, 30)
            )
            .unwrap()
    );
}

/// A simple function to test the i32.shr_u bitwise operation
#[test_log::test]
pub fn i32_bitwise_shr_u() {
    let wat = String::from(BASE_WAT).replace("{{0}}", "shr_u");

    let wasm_bytes = wat::parse_str(wat).unwrap();

    let validation_info = validate(&wasm_bytes).expect("validation failed");

    let mut instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    assert_eq!(
        8881445,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (142_103_123, 4)
            )
            .unwrap()
    );
    assert_eq!(
        23879,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (391_248_921, 14)
            )
            .unwrap()
    );
    assert_eq!(
        601955006,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (1_203_910_012, 33)
            )
            .unwrap()
    );
    assert_eq!(
        1056594615,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (2_113_189_231, 33)
            )
            .unwrap()
    );
    assert_eq!(
        1,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MIN, i32::MAX)
            )
            .unwrap()
    );

    // Basic positive number
    assert_eq!(
        4,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (8, 1))
            .unwrap()
    );

    // Shifting by 0 (no shift)
    assert_eq!(
        -1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-1, 0))
            .unwrap()
    );
    assert_eq!(
        1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (1, 0))
            .unwrap()
    );

    // Shifting negative numbers
    assert_eq!(
        i32::MAX - 3,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-8, 1))
            .unwrap()
    );
    assert_eq!(
        i32::MAX,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-1, 1))
            .unwrap()
    );

    // Shifting by 31 (maximum shift for 32-bit int)
    assert_eq!(
        1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-1, 31))
            .unwrap()
    );
    assert_eq!(
        1,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MIN, 31)
            )
            .unwrap()
    );
    assert_eq!(
        0,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MAX, 31)
            )
            .unwrap()
    );

    // Shifting by more than 31
    assert_eq!(
        -1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-1, 32))
            .unwrap()
    );
    assert_eq!(
        1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (1, 32))
            .unwrap()
    );
    assert_eq!(
        268435455,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-1, 100))
            .unwrap()
    );

    // Minimum and maximum 32-bit integers
    assert_eq!(
        (i32::MIN / 2) * (-1),
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MIN, 1)
            )
            .unwrap()
    );
    assert_eq!(
        i32::MAX / 2,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MAX, 1)
            )
            .unwrap()
    );

    // Shifting out all bits except sign
    assert_eq!(
        2,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MIN, 30)
            )
            .unwrap()
    );
    assert_eq!(
        1,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MAX, 30)
            )
            .unwrap()
    );
}

/// A simple function to test the i32.rotl bitwise operation
#[test_log::test]
pub fn i32_bitwise_rotl() {
    let wat = String::from(BASE_WAT).replace("{{0}}", "rotl");

    let wasm_bytes = wat::parse_str(wat).unwrap();

    let validation_info = validate(&wasm_bytes).expect("validation failed");

    let mut instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    assert_eq!(
        -2021317328,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (142_103_123, 4)
            )
            .unwrap()
    );
    assert_eq!(
        2131117524,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (391_248_921, 14)
            )
            .unwrap()
    );
    assert_eq!(
        -1887147272,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (1_203_910_012, 33)
            )
            .unwrap()
    );
    assert_eq!(
        -68588834,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (2_113_189_231, 33)
            )
            .unwrap()
    );
    assert_eq!(
        1073741824,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MIN, i32::MAX)
            )
            .unwrap()
    );

    // Basic positive number
    assert_eq!(
        16,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (8, 1))
            .unwrap()
    );

    // Rotating by 0 (no shift)
    assert_eq!(
        -1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-1, 0))
            .unwrap()
    );
    assert_eq!(
        1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (1, 0))
            .unwrap()
    );

    // Shifting negative numbers
    assert_eq!(
        -15,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-8, 1))
            .unwrap()
    );
    assert_eq!(
        -1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-1, 1))
            .unwrap()
    );

    // Rotating by 31
    assert_eq!(
        -1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-1, 31))
            .unwrap()
    );
    assert_eq!(
        i32::MAX / 2 + 1,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MIN, 31)
            )
            .unwrap()
    );
    assert_eq!(
        i32::MIN / 2 - 1,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MAX, 31)
            )
            .unwrap()
    );

    // Rotating by more than 31
    assert_eq!(
        -1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-1, 32))
            .unwrap()
    );
    assert_eq!(
        1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (1, 32))
            .unwrap()
    );
    assert_eq!(
        -1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-1, 100))
            .unwrap()
    );

    // Minimum and maximum 32-bit integers
    assert_eq!(
        1,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MIN, 1)
            )
            .unwrap()
    );
    assert_eq!(
        -2,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MAX, 1)
            )
            .unwrap()
    );

    // Shifting out all bits except sign
    assert_eq!(
        i32::MAX / 4 + 1,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MIN, 30)
            )
            .unwrap()
    );
    assert_eq!(
        i32::MIN / 4 - 1,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MAX, 30)
            )
            .unwrap()
    );
}

/// A simple function to test the i32.rotr bitwise operation
#[test_log::test]
pub fn i32_bitwise_rotr() {
    let wat = String::from(BASE_WAT).replace("{{0}}", "rotr");

    let wasm_bytes = wat::parse_str(wat).unwrap();

    let validation_info = validate(&wasm_bytes).expect("validation failed");

    let mut instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    assert_eq!(
        814187813,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (142_103_123, 4)
            )
            .unwrap()
    );
    assert_eq!(
        -261857977,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (391_248_921, 14)
            )
            .unwrap()
    );
    assert_eq!(
        601955006,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (1_203_910_012, 33)
            )
            .unwrap()
    );
    assert_eq!(
        -1090889033,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (2_113_189_231, 33)
            )
            .unwrap()
    );
    assert_eq!(
        1,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MIN, i32::MAX)
            )
            .unwrap()
    );

    // Basic positive number
    assert_eq!(
        4,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (8, 1))
            .unwrap()
    );

    // Rotating by 0 (no shift)
    assert_eq!(
        -1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-1, 0))
            .unwrap()
    );
    assert_eq!(
        1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (1, 0))
            .unwrap()
    );

    // Shifting negative numbers
    assert_eq!(
        i32::MAX - 3,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-8, 1))
            .unwrap()
    );
    assert_eq!(
        -1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-1, 1))
            .unwrap()
    );

    // Rotating by 31
    assert_eq!(
        -1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-1, 31))
            .unwrap()
    );
    assert_eq!(
        1,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MIN, 31)
            )
            .unwrap()
    );
    assert_eq!(
        -2,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MAX, 31)
            )
            .unwrap()
    );

    // Rotating by more than 31
    assert_eq!(
        -1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-1, 32))
            .unwrap()
    );
    assert_eq!(
        1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (1, 32))
            .unwrap()
    );
    assert_eq!(
        -1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), (-1, 100))
            .unwrap()
    );

    // Minimum and maximum 32-bit integers
    assert_eq!(
        i32::MAX / 2 + 1,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MIN, 1)
            )
            .unwrap()
    );
    assert_eq!(
        i32::MIN / 2 - 1,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MAX, 1)
            )
            .unwrap()
    );

    // Shifting out all bits except sign
    assert_eq!(
        2,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MIN, 30)
            )
            .unwrap()
    );
    assert_eq!(
        -3,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i32::MAX, 30)
            )
            .unwrap()
    );
}

/// A simple function to test the i32.clz bitwise operation
#[test_log::test]
pub fn i32_bitwise_clz() {
    let wat = String::from(BASE_COUNT_WAT).replace("{{0}}", "clz");

    let wasm_bytes = wat::parse_str(wat).unwrap();

    let validation_info = validate(&wasm_bytes).expect("validation failed");

    let mut instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    assert_eq!(
        26,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), 33)
            .unwrap()
    );
    assert_eq!(
        25,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), 77)
            .unwrap()
    );
    assert_eq!(
        14,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), 192534)
            .unwrap()
    );
    assert_eq!(
        0,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), i32::MIN)
            .unwrap()
    );
    assert_eq!(
        1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), i32::MAX)
            .unwrap()
    );
    assert_eq!(
        32,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), 0)
            .unwrap()
    );
}

/// A simple function to test the i32.ctz bitwise operation
#[test_log::test]
pub fn i32_bitwise_ctz() {
    let wat = String::from(BASE_COUNT_WAT).replace("{{0}}", "ctz");

    let wasm_bytes = wat::parse_str(wat).unwrap();

    let validation_info = validate(&wasm_bytes).expect("validation failed");

    let mut instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    assert_eq!(
        0,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), 33)
            .unwrap()
    );
    assert_eq!(
        0,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), 77)
            .unwrap()
    );
    assert_eq!(
        1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), 192534)
            .unwrap()
    );
    assert_eq!(
        31,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), i32::MIN)
            .unwrap()
    );
    assert_eq!(
        0,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), i32::MAX)
            .unwrap()
    );
    assert_eq!(
        32,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), 0)
            .unwrap()
    );
}

/// A simple function to test the i32.popcnt bitwise operation
#[test_log::test]
pub fn i32_bitwise_popcnt() {
    let wat = String::from(BASE_COUNT_WAT).replace("{{0}}", "popcnt");

    let wasm_bytes = wat::parse_str(wat).unwrap();

    let validation_info = validate(&wasm_bytes).expect("validation failed");

    let mut instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    assert_eq!(
        2,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), 33)
            .unwrap()
    );
    assert_eq!(
        4,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), 77)
            .unwrap()
    );
    assert_eq!(
        8,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), 192534)
            .unwrap()
    );
    assert_eq!(
        1,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), i32::MIN)
            .unwrap()
    );
    assert_eq!(
        31,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), i32::MAX)
            .unwrap()
    );
    assert_eq!(
        0,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), 0)
            .unwrap()
    );
}

const I64_BASE_WAT: &'static str = r#"
    (module
      (func (export "template") (param $x i64) (param $y i64) (result i64)
          local.get $x
          local.get $y
          i64.{{0}})
    )
"#;

const I64_BASE_COUNT_WAT: &'static str = r#"
    (module
      (func (export "template") (param $x i64) (result i64)
          local.get $x
          i64.{{0}})
    )
"#;

/// A simple function to test the i64.and bitwise operation
#[test_log::test]
pub fn i64_bitwise_and() {
    let wat = String::from(I64_BASE_WAT).replace("{{0}}", "and");

    let wasm_bytes = wat::parse_str(wat).unwrap();

    let validation_info = validate(&wasm_bytes).expect("validation failed");

    let mut instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    assert_eq!(
        1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (33 as i64, 11 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        5 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (77 as i64, 23 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        180244 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (192534 as i64, 1231412 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        0 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MIN, i64::MAX)
            )
            .unwrap()
    );
}

/// A simple function to test the i64.or bitwise operation
#[test_log::test]
pub fn i64_bitwise_or() {
    let wat = String::from(I64_BASE_WAT).replace("{{0}}", "or");

    let wasm_bytes = wat::parse_str(wat).unwrap();

    let validation_info = validate(&wasm_bytes).expect("validation failed");

    let mut instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    assert_eq!(
        43 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (33 as i64, 11 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        95 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (77 as i64, 23 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        1243702 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (192534 as i64, 1231412 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        -1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MIN, i64::MAX)
            )
            .unwrap()
    );
}

/// A simple function to test the i64.xor bitwise operation
#[test_log::test]
pub fn i64_bitwise_xor() {
    let wat = String::from(I64_BASE_WAT).replace("{{0}}", "xor");

    let wasm_bytes = wat::parse_str(wat).unwrap();

    let validation_info = validate(&wasm_bytes).expect("validation failed");

    let mut instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    assert_eq!(
        42 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (33 as i64, 11 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        90 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (77 as i64, 23 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        1063458 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (192534 as i64, 1231412 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        -1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MIN, i64::MAX)
            )
            .unwrap()
    );
}

/// A simple function to test the i64.shl bitwise operation
#[test_log::test]
pub fn i64_bitwise_shl() {
    let wat = String::from(I64_BASE_WAT).replace("{{0}}", "shl");

    let wasm_bytes = wat::parse_str(wat).unwrap();

    let validation_info = validate(&wasm_bytes).expect("validation failed");

    let mut instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    assert_eq!(
        67584 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (33 as i64, 11 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        645922816 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (77 as i64, 23 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        99079191802150912 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (192534 as i64, 1231412 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        0 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MIN, i64::MAX)
            )
            .unwrap()
    );
}

/// A simple function to test the i64.shr_s bitwise operation
#[test_log::test]
pub fn i64_bitwise_shr_s() {
    let wat = String::from(I64_BASE_WAT).replace("{{0}}", "shr_s");

    let wasm_bytes = wat::parse_str(wat).unwrap();

    let validation_info = validate(&wasm_bytes).expect("validation failed");

    let mut instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    assert_eq!(
        8881445 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (142_103_123 as i64, 4 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        23879 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (391_248_921 as i64, 14 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        0 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (1_203_910_012 as i64, 33 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        0 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (2_113_189_231 as i64, 33 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        -1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MIN, i64::MAX)
            )
            .unwrap()
    );

    // Basic positive number
    assert_eq!(
        4 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (8 as i64, 1 as i64)
            )
            .unwrap()
    );

    // Shifting by 0 (no shift)
    assert_eq!(
        -1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-1 as i64, 0 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (1 as i64, 0 as i64)
            )
            .unwrap()
    );

    // Shifting negative numbers
    assert_eq!(
        -4 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-8 as i64, 1 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        -1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-1 as i64, 1 as i64)
            )
            .unwrap()
    );

    // Shifting by 31 (maximum shift for 32-bit int)
    assert_eq!(
        -1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-1 as i64, 31 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        -4294967296 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MIN, 31 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        4294967295 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MAX, 31 as i64)
            )
            .unwrap()
    );

    // Shifting by more than 31
    assert_eq!(
        -1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-1 as i64, 32 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        0 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (1 as i64, 32 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        -1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-1 as i64, 100 as i64)
            )
            .unwrap()
    );

    // Minimum and maximum 32-bit integers
    assert_eq!(
        i64::MIN / 2,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MIN, 1 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        i64::MAX / 2,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MAX, 1 as i64)
            )
            .unwrap()
    );

    // Shifting out all bits except sign
    assert_eq!(
        -8589934592 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MIN, 30 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        8589934591 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MAX, 30 as i64)
            )
            .unwrap()
    );
}

/// A simple function to test the i64.shr_u bitwise operation
#[test_log::test]
pub fn i64_bitwise_shr_u() {
    let wat = String::from(I64_BASE_WAT).replace("{{0}}", "shr_u");

    let wasm_bytes = wat::parse_str(wat).unwrap();

    let validation_info = validate(&wasm_bytes).expect("validation failed");

    let mut instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    assert_eq!(
        8881445 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (142_103_123 as i64, 4 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        23879 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (391_248_921 as i64, 14 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        0 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (1_203_910_012 as i64, 33 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        0 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (2_113_189_231 as i64, 33 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MIN, i64::MAX)
            )
            .unwrap()
    );

    // Basic positive number
    assert_eq!(
        4 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (8 as i64, 1 as i64)
            )
            .unwrap()
    );

    // Shifting by 0 (no shift)
    assert_eq!(
        -1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-1 as i64, 0 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (1 as i64, 0 as i64)
            )
            .unwrap()
    );

    // Shifting negative numbers
    assert_eq!(
        i64::MAX - 3,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-8 as i64, 1 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        i64::MAX,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-1 as i64, 1 as i64)
            )
            .unwrap()
    );

    // Shifting by 31 (maximum shift for 32-bit int)
    assert_eq!(
        8589934591 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-1 as i64, 31 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        4294967296 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MIN, 31 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        4294967295 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MAX, 31 as i64)
            )
            .unwrap()
    );

    // Shifting by more than 31
    assert_eq!(
        4294967295 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-1 as i64, 32 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        0 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (1 as i64, 32 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        268435455 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-1 as i64, 100 as i64)
            )
            .unwrap()
    );

    // Minimum and maximum 32-bit integers
    assert_eq!(
        (i64::MIN / 2) * (-1),
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MIN, 1 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        i64::MAX / 2,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MAX, 1 as i64)
            )
            .unwrap()
    );

    // Shifting out all bits except sign
    assert_eq!(
        8589934592 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MIN, 30 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        8589934591 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MAX, 30 as i64)
            )
            .unwrap()
    );
}

/// A simple function to test the i64.rotl bitwise operation
#[test_log::test]
pub fn i64_bitwise_rotl() {
    let wat = String::from(I64_BASE_WAT).replace("{{0}}", "rotl");

    let wasm_bytes = wat::parse_str(wat).unwrap();

    let validation_info = validate(&wasm_bytes).expect("validation failed");

    let mut instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    assert_eq!(
        2273649968 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (142_103_123 as i64, 4 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        6410222321664 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (391_248_921 as i64, 14 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        -8105235815975616512 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (1_203_910_012 as i64, 33 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        -294586798900772864 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (2_113_189_231 as i64, 33 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        4611686018427387904 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MIN, i64::MAX)
            )
            .unwrap()
    );

    // Basic positive number
    assert_eq!(
        16 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (8 as i64, 1 as i64)
            )
            .unwrap()
    );

    // Rotating by 0 (no shift)
    assert_eq!(
        -1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-1 as i64, 0 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (1 as i64, 0 as i64)
            )
            .unwrap()
    );

    // Shifting negative numbers
    assert_eq!(
        -15 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-8 as i64, 1 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        -1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-1 as i64, 1 as i64)
            )
            .unwrap()
    );

    // Rotating by 31
    assert_eq!(
        -1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-1 as i64, 31 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        1073741824 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MIN, 31 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        -1073741825 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MAX, 31 as i64)
            )
            .unwrap()
    );

    // Rotating by more than 31
    assert_eq!(
        -1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-1 as i64, 32 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        4294967296 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (1 as i64, 32 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        -1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-1 as i64, 100 as i64)
            )
            .unwrap()
    );

    // Minimum and maximum 32-bit integers
    assert_eq!(
        1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MIN, 1 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        -2 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MAX, 1 as i64)
            )
            .unwrap()
    );

    // Shifting out all bits except sign
    assert_eq!(
        536870912 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MIN, 30 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        -536870913 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MAX, 30 as i64)
            )
            .unwrap()
    );
}

/// A simple function to test the i64.rotr bitwise operation
#[test_log::test]
pub fn i64_bitwise_rotr() {
    let wat = String::from(I64_BASE_WAT).replace("{{0}}", "rotr");

    let wasm_bytes = wat::parse_str(wat).unwrap();

    let validation_info = validate(&wasm_bytes).expect("validation failed");

    let mut instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    assert_eq!(
        3458764513829422373 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (142_103_123 as i64, 4 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        -1124774006935757497 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (391_248_921 as i64, 14 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        2585377064433483776 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (1_203_910_012 as i64, 33 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        4538039318702194688 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (2_113_189_231 as i64, 33 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MIN, i64::MAX)
            )
            .unwrap()
    );

    // Basic positive number
    assert_eq!(
        4 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (8 as i64, 1 as i64)
            )
            .unwrap()
    );

    // Rotating by 0 (no shift)
    assert_eq!(
        -1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-1 as i64, 0 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (1 as i64, 0 as i64)
            )
            .unwrap()
    );

    // Shifting negative numbers
    assert_eq!(
        i64::MAX - 3,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-8 as i64, 1 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        -1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-1 as i64, 1 as i64)
            )
            .unwrap()
    );

    // Rotating by 31
    assert_eq!(
        -1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-1 as i64, 31 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        4294967296 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MIN, 31 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        -4294967297 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MAX, 31 as i64)
            )
            .unwrap()
    );

    // Rotating by more than 31
    assert_eq!(
        -1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-1 as i64, 32 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        4294967296 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (1 as i64, 32 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        -1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (-1 as i64, 100 as i64)
            )
            .unwrap()
    );

    // Minimum and maximum 32-bit integers
    assert_eq!(
        i64::MAX / 2 + 1,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MIN, 1 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        i64::MIN / 2 - 1,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MAX, 1 as i64)
            )
            .unwrap()
    );

    // Shifting out all bits except sign
    assert_eq!(
        8589934592 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MIN, 30 as i64)
            )
            .unwrap()
    );
    assert_eq!(
        -8589934593 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                (i64::MAX, 30 as i64)
            )
            .unwrap()
    );
}

/// A simple function to test the i64.clz bitwise operation
#[test_log::test]
pub fn i64_bitwise_clz() {
    let wat = String::from(I64_BASE_COUNT_WAT).replace("{{0}}", "clz");

    let wasm_bytes = wat::parse_str(wat).unwrap();

    let validation_info = validate(&wasm_bytes).expect("validation failed");

    let mut instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    assert_eq!(
        58 as i64,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), 33 as i64)
            .unwrap()
    );
    assert_eq!(
        57 as i64,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), 77 as i64)
            .unwrap()
    );
    assert_eq!(
        46 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                192534 as i64
            )
            .unwrap()
    );
    assert_eq!(
        0 as i64,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), i64::MIN)
            .unwrap()
    );
    assert_eq!(
        1 as i64,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), i64::MAX)
            .unwrap()
    );
    assert_eq!(
        64 as i64,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), 0 as i64)
            .unwrap()
    );
}

/// A simple function to test the i64.ctz bitwise operation
#[test_log::test]
pub fn i64_bitwise_ctz() {
    let wat = String::from(I64_BASE_COUNT_WAT).replace("{{0}}", "ctz");

    let wasm_bytes = wat::parse_str(wat).unwrap();

    let validation_info = validate(&wasm_bytes).expect("validation failed");

    let mut instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    assert_eq!(
        0 as i64,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), 33 as i64)
            .unwrap()
    );
    assert_eq!(
        0 as i64,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), 77 as i64)
            .unwrap()
    );
    assert_eq!(
        1 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                192534 as i64
            )
            .unwrap()
    );
    assert_eq!(
        63 as i64,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), i64::MIN)
            .unwrap()
    );
    assert_eq!(
        0 as i64,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), i64::MAX)
            .unwrap()
    );
    assert_eq!(
        64 as i64,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), 0 as i64)
            .unwrap()
    );
}

/// A simple function to test the i64.popcnt bitwise operation
#[test_log::test]
pub fn i64_bitwise_popcnt() {
    let wat = String::from(I64_BASE_COUNT_WAT).replace("{{0}}", "popcnt");

    let wasm_bytes = wat::parse_str(wat).unwrap();

    let validation_info = validate(&wasm_bytes).expect("validation failed");

    let mut instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    assert_eq!(
        2 as i64,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), 33 as i64)
            .unwrap()
    );
    assert_eq!(
        4 as i64,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), 77 as i64)
            .unwrap()
    );
    assert_eq!(
        8 as i64,
        instance
            .invoke(
                &instance.get_function_by_index(0, 0).unwrap(),
                192534 as i64
            )
            .unwrap()
    );
    assert_eq!(
        1 as i64,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), i64::MIN)
            .unwrap()
    );
    assert_eq!(
        63 as i64,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), i64::MAX)
            .unwrap()
    );
    assert_eq!(
        0 as i64,
        instance
            .invoke(&instance.get_function_by_index(0, 0).unwrap(), 0 as i64)
            .unwrap()
    );
}
