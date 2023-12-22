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
