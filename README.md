# enum_is

Procedural macro that generates `is_*` predicate methods for enum variants.

With `#[derive(EnumIs)]`, every variant of your enum gets an `is_<variant>()`
method (in `snake_case`) that returns `true` when `self` matches that variant.

Works with unit, tuple, and struct variants.

Options:

- Skip a variant with `#[enum_is(ignore)]`.
- Override a method name with `#[enum_is(rename = "...")]`.
- Generate a shared predicate with `#[enum_is(group = "...")]`.
- `group` and `rename` can be combined in one `#[enum_is(...)]` (or separate ones); `ignore` must stand alone.

## Install

Add to `Cargo.toml`:

```toml
[dependencies]
enum_is = "0.2"
```

Import the derive:

```rust
use enum_is::EnumIs;
```

## Example

```rust
use enum_is::EnumIs;

#[derive(EnumIs, Debug)]
enum Mode {
    Fast,
    Normal,
    Slow,
}

fn handle(mode: Mode) {
    if mode.is_fast() {
        // handle fast mode
    }
}
```

Generated methods look like:

```rust,ignore
impl Mode {
    pub fn is_fast(&self) -> bool { /* ... */ }
    pub fn is_normal(&self) -> bool { /* ... */ }
    pub fn is_slow(&self) -> bool { /* ... */ }
}
```

## Ignoring variants

Use `#[enum_is(ignore)]` on any variant you don’t want a predicate for:

```rust
use enum_is::EnumIs;

#[derive(EnumIs)]
enum MaybeNumber {
    Int(i32),
    #[enum_is(ignore)]
    NotNumber,
}
// v.is_not_number() is NOT generated
```

## Renaming methods

Use `#[enum_is(rename = "...")]` on a variant to override its method name:

```rust
use enum_is::EnumIs;

#[derive(EnumIs)]
enum Drive {
    #[enum_is(rename = "is_dos")]
    DriveOrStop,
}

let o = Drive::DriveOrStop;
assert!(o.is_dos());
```

## Grouping variants

Use `#[enum_is(group = "...")]` to generate a shared predicate for multiple variants. The group name becomes the method name.

```rust
use enum_is::EnumIs;

#[derive(EnumIs)]
enum Message {
    Ping,
    #[enum_is(group = "is_payload")]
    Data { id: u64, payload: Vec<u8> },
    #[enum_is(group = "is_payload")]
    Binary(Vec<u8>),
}

let m = Message::Binary(vec![]);
assert!(m.is_payload());
assert!(!m.is_ping());
```

## Supported enums

`EnumIs` works with:

- **Unit variants**

  ```rust
  use enum_is::EnumIs;

  #[derive(EnumIs)]
  enum Status {
      Ok,
      Error,
  }

  let s = Status::Ok;
  assert!(s.is_ok());
  assert!(!s.is_error());
  ```

- **Tuple variants**

  ```rust
  use enum_is::EnumIs;

  #[derive(EnumIs)]
  enum Value {
      Int(i32),
      Pair(u8, u8),
  }

  let v = Value::Pair(1, 2);
  assert!(v.is_pair());
  assert!(!v.is_int());
  ```

- **Struct variants**

  ```rust
  use enum_is::EnumIs;

  #[derive(EnumIs)]
  enum Message {
      Ping,
      Data { id: u64, payload: Vec<u8> },
  }

  let m = Message::Data { id: 1, payload: vec![] };
  assert!(m.is_data());
  assert!(!m.is_ping());
  ```

## Naming rules

For each variant, `EnumIs` generates a method:

- Method name: `is_<variant_name_in_snake_case>`
- Example mappings:

  | Variant name       | Method name               |
  | ------------------ | ------------------------- |
  | `Fast`             | `is_fast()`               |
  | `PostOnly`         | `is_post_only()`          |
  | `CPU`              | `is_cpu()`                |
  | `HTTPRequestError` | `is_http_request_error()` |

The methods take `&self` and return `bool`.

Internally, the macro uses `matches!` with:

- `Self::Variant` for unit variants
- `Self::Variant(..)` for tuple variants
- `Self::Variant { .. }` for struct variants

## Limitations

- `#[derive(EnumIs)]` must be used on **enums** only.
