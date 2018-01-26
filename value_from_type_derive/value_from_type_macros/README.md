Value from Type -Macros
===============================

[![Latest Version](https://img.shields.io/crates/v/value_from_type_macros.svg)](https://crates.io/crates/value_from_type_macros)
[![Rust Documentation](https://docs.rs/value_from_type_macros/badge.svg)](https://docs.rs/value_from_type_macros)

Procedural macro attribute to match structure types with an enum variant.

This macro can be applied on a module to make a connection between each defined struct
and a newly created enum type. This enum is built into the same module as 
the macro is invocated upon.
The macro will also implement [value_from_type_traits::FromType](https://docs.rs/value_from_type_traits) on the enum
for each struct (within the module) as generic argument.

# Examples
 
```rust
#![feature(proc_macro)]
extern crate value_from_type_macros;
extern crate value_from_type_traits;

// Attribute macro must be imported through a use statement.
use value_from_type_macros::value_from_type;
// Implemented trait on `EnumName`
use value_from_type_traits::IntoEnum;

mod temp {
    // The parameter indicates the enum identifier.
    #![value_from_type(EnumName)]

    #[derive(Debug)]
    pub struct X(); 
 
// Explicit import for sake of example.
use self::temp::{EnumName, X};
// use self::temp::*;

fn main() {
	assert_eq!(EnumName::X, X::into_enum()); 
}
```
