# BitSet Library

## Introduction
**bitset** is a crate introducing a data type representing sets of bits.

## Getting Started
To use the library as a dependency in your project, add **bitset** to your 
`Cargo.toml` file

```ignore
[dependencies]
bitset = "0.2.4"
```

After that, place the crate declaration in your project's `main.rs` 
or `lib.rs` file

```rust
extern crate bitset;
```

## Features
**bitset** is a library adds the `BitSet` data type to a system. This allows
one to use a sequence of arbitrary many bits and apply standard logic operations
to them. One possible use case would be for tracking data dependencies between 
subsystems in a larger software system using the bit set to track what data 
components each data type has a in data-oriented design fashion. One can apply
the standard bitwise logic operations to bit sets such as logical AND, logical 
OR, logical XOR, SHIFT LEFT, SHIFT RIGHT, and logical NEGATION. One can also 
query, test, set and flip individual bits.

## Limitations
The main limitation of the **bitset** crate is that it only supports a bit set 
capacity of 128 bits. This is the largest unsigned integer that Rust currently 
has. This limitation will be removed in the future when Rust gets const 
generics.

