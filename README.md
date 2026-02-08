# rumbok

**rumbok** is a Lombok-inspired derive macro library for Rust.

It provides `Getter`, `Setter`, and `Data` derive macros to reduce boilerplate code,
while keeping Rust's ownership and borrowing semantics explicit and safe.

---

## ✨ Features

- `#[derive(Setter)]`
  - Generates `set_xxx(&mut self, value: T)` methods
- `#[derive(Getter)]`
  - Generates `get_xxx(&self) -> &T` / `&str` methods
- `#[derive(Data)]`
  - Generates both Getter and Setter
- Field-level attribute controls:
  - `#[getter(skip)]` – Skip generating a getter for the field
  - `#[setter(skip)]` – Skip generating a setter for the field
  - `#[getter(clone)]` – Generate a getter that returns an owned value (`T`)
- Supports generics, lifetimes, and `where` clauses
- Correctly handles owned types and reference types
- UI tests using `trybuild` (including compile-fail cases)

---

## 📦 Installation

```toml
[dependencies]
rumbok = "0.1"
```

---

## 🚀 Basic Usage

### Data (Getter + Setter)

```rust
use rumbok::Data;

#[derive(Data)]
struct User<'a> {
    id: i32,
    #[getter(skip)]
    name: String,
    reference: &'a str,
    #[getter(clone)]
    err: Result<i32,String>,
}

fn main() {
    let mut u = User {
        id: 1,
        name: "Alice".into(),
        reference: "ref",
        err: Ok(1),
    };

    u.set_id(10);
    u.set_name("Bob".into());
    u.set_reference("new_ref");
    u.set_err(Ok(2));

    assert_eq!(10, *u.get_id());
    // ❌ compile error: no method named `get_name`
    // assert_eq!("Bob", u.get_name());
    assert_eq!("new_ref", u.get_reference());
    let expected_err: Result<i32, String> = Ok(1);
    assert_eq!(expected_err, u.get_err());
}
```

---

## 🧩 Individual Derives

### Setter

```rust
use rumbok::Setter;

#[derive(Setter)]
struct User {
    id: i32,
    name: String,
}

fn main() {
    let mut u = User { id: 0, name: "A".into() };
    u.set_id(1);
    u.set_name("B".into());
}
```

#### Skip setter generation

You can skip generating a setter for a specific field using `#[setter(skip)]`:

```rust
use rumbok::Setter;

#[derive(Setter)]
struct User {
    id: i32,
    #[setter(skip)]
    internal: String,
}

fn main() {
    let mut u = User {
        id: 1,
        internal: "secret".into(),
    };

    u.set_id(10);

    // ❌ compile error: no method named `set_internal`
    // u.set_internal("x".into());
}
```

---

### Getter

```rust
use rumbok::Getter;

#[derive(Getter)]
struct User<'a> {
    id: i32,
    name: String,
    reference: &'a str,
}

fn main() {
    let u = User {
        id: 1,
        name: "A".into(),
        reference: "ref",
    };

    let _: &i32 = u.get_id();
    let _: &String = u.get_name();
    let _: &str = u.get_reference();
}
```

Getter rules:

| Field Type | Return Type |
|-----------|-------------|
| `T` | `&T` |
| `&T` | `&T` |
| `&mut T` | `&T` |

#### Skip getter generation

You can skip generating a getter for a specific field using `#[getter(skip)]`:

```rust
use rumbok::Getter;

#[derive(Getter)]
struct User {
    id: i32,
    #[getter(skip)]
    password: String,
}

fn main() {
    let u = User {
        id: 1,
        password: "secret".into(),
    };

    let _ = u.get_id();

    // ❌ compile error: no method named `get_password`
    // u.get_password();
}
```

#### Getter by value (`clone`)

By default, getters return references.

Using `#[getter(clone)]`, you can generate a getter that returns an owned value:

```rust
use rumbok::Getter;

#[derive(Getter)]
struct User {
    #[getter(clone)]
    name: String,
}

fn main() {
    let u = User { name: "Alice".into() };

    let name: String = u.get_name();
}
```

Notes:

- The field type **must implement `Clone`**
- If the type does not implement `Clone`, a compile-time error will occur

---

## 🧠 Design Philosophy

- Do not hide Rust's ownership and borrowing rules
- Prefer explicit and predictable APIs
- Avoid guessing trait implementations such as `Copy`
- Attribute options such as `skip` and `clone` are explicit and never inferred

Getters always return references by design unless explicitly specified.

---

## ⚠️ Limitations

- Tuple structs and unit structs are not supported
- Enum types are not supported
- No automatic `Copy` detection

---

## 🧪 Testing

This crate uses:

- Unit tests for macro expansion
- UI tests with `trybuild`

```bash
cargo test
```

---

## 🔍 Debugging Macros

To inspect expanded macros, use `cargo-expand`:

```bash
cargo install cargo-expand
cargo expand
```

---

## 📄 License

Licensed under either of:

- MIT License

---

## 🙌 Motivation

Inspired by Java's Lombok, this project aims to:

- Reduce boilerplate
- Preserve Rust's safety guarantees
- Provide simple and predictable derive macros

---

## 🚧 Roadmap

- `#[getter(by_value)]`
- Option-aware getters
- `#[derive(Value)]`
- `#[derive(Constructor)]`

---

Enjoy writing less boilerplate with **rumbok** ✨

