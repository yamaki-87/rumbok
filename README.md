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
    name: String,
    reference: &'a str,
}

fn main() {
    let mut u = User {
        id: 1,
        name: "Alice".into(),
        reference: "ref",
    };

    u.set_id(10);
    u.set_name("Bob".into());
    u.set_reference("new_ref");

    assert_eq!(10, *u.get_id());
    assert_eq!("Bob", u.get_name());
    assert_eq!("new_ref", u.get_reference());
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

---

## 🧠 Design Philosophy

- Do not hide Rust's ownership and borrowing rules
- Prefer explicit and predictable APIs
- Avoid guessing trait implementations such as `Copy`

Getters always return references by design.

---

## ⚠️ Limitations

- Tuple structs and unit structs are not supported
- Field-level attributes are not yet implemented
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
- Apache License, Version 2.0

---

## 🙌 Motivation

Inspired by Java's Lombok, this project aims to:

- Reduce boilerplate
- Preserve Rust's safety guarantees
- Provide simple and predictable derive macros

---

## 🚧 Roadmap

- `#[getter(skip)]`, `#[setter(skip)]`
- `#[getter(by_value)]`
- Option-aware getters
- `#[derive(Value)]`
- `#[derive(Constructor)]`

---

Enjoy writing less boilerplate with **rumbok** ✨
