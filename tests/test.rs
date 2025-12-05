use enum_is::EnumIs;

#[allow(dead_code)]
#[derive(EnumIs, Debug)]
enum Mixed {
    Unit,
    Tuple(u8, u8),
    Struct { x: i32, y: i32 },
}

#[test]
fn unit_variant() {
    let v = Mixed::Unit;

    assert!(!v.is_tuple());
    assert!(v.is_unit());
    assert!(!v.is_struct());
}

#[test]
fn tuple_variant() {
    let v = Mixed::Tuple(1, 2);

    assert!(v.is_tuple());
    assert!(!v.is_unit());
    assert!(!v.is_struct());
}

#[test]
fn struct_variant() {
    let v = Mixed::Struct { x: 10, y: 20 };

    assert!(v.is_struct());
    assert!(!v.is_unit());
    assert!(!v.is_tuple());
}
