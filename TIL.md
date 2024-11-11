In Rust, calling a **trait** method on a type
is equivalent to invoking a function.

e.g.

```rust
let a: i32 = 5;

// these are equivalent
let b = a.clone();
let b = Clone::clone(&a);
```

The `clone` method takes a reference, and this borrow happens implicitly in the "method".

```rust
let a: i32 = 5;

// these are equivalent
let b = a.clone();
let b = &a.clone(); // <-- new
let b = Clone::clone(&a);

// let b = Clone::clone(a); // <-- but: this does NOT borrow implicitly
```

Thanks, https://fasterthanli.me/articles/a-half-hour-to-learn-rust !

--

Building a universal binary for Mac:

```
rustup target add x86_64-apple-darwin
cargo install universal2
cargo universal2
```

This outputs a universal binary at `./target/universal2-apple-darwin/release/drum-break`.

For more info: https://github.com/randomairborne/cargo-universal2/blob/main/src/lib.rs
