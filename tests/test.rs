use enum_is::EnumIs;

#[allow(dead_code)]
#[derive(EnumIs, Debug)]
enum Mixed {
    #[enum_is(group = "is_uts")]
    Unit,
    #[enum_is(group = "is_uts")]
    Tuple(u8, u8),
    #[enum_is(group = "is_uts")]
    Struct { x: i32, y: i32 },
    #[enum_is(ignore)]
    Ignored,
    #[enum_is(group = "is_uts", rename = "is_renamed")]
    Named,
}

#[test]
fn unit_variant() {
    let v = Mixed::Unit;

    assert!(!v.is_tuple());
    assert!(v.is_unit());
    assert!(!v.is_struct());
    assert!(v.is_uts());
}

#[test]
fn tuple_variant() {
    let v = Mixed::Tuple(1, 2);

    assert!(v.is_tuple());
    assert!(!v.is_unit());
    assert!(!v.is_struct());
    assert!(v.is_uts());
}

#[test]
fn struct_variant() {
    let v = Mixed::Struct { x: 10, y: 20 };
    assert!(v.is_struct());
    assert!(!v.is_unit());
    assert!(!v.is_tuple());
    assert!(v.is_uts());
}

#[test]
fn rename_and_group() {
    let v = Mixed::Named;
    assert!(v.is_renamed());
    assert!(v.is_uts());
}
