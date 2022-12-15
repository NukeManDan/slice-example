# slice-example

```rust
/// Consider this Rust code:

fn sort(items: &mut [u32]) {
    // Implementation goes here
}

///
/// The signature above makes sense for a sorting algorithm that:
///
/// - a) Returns a copy of the data in a new Vec in sorted order.
/// - b) Returns a copy of the data in a new slice in sorted order.
/// - c) Returns a new subslice of the same data in sorted order
/// - d) Sorts the data "in place" by moving elements around.
/// - e) Sorts the data and removes duplicate items.
```

https://doc.rust-lang.org/rust-by-example/primitives/array.html helps us understand what is going on here:
`fn sort(items: &mut [u32])` is acting on a slice.

> Slices can be used to borrow a section of an array, and have the type signature &[T].

We also make notice it's mutable.
But on trying to manipulate the slice you obtain, you cannot (AFAIK) safely change the _length_ of the slice you borrowed.
It makes sense that you should be barred from doing so by the borrow checker.

**Let's try to prove `e` Using given fn signature and https://docs.rs/resize-slice/latest/resize_slice/ .**

---

At commit `59db3d25964138b128bc82d7d197cc696c519727` (see [main.rs](./src/main.rs)) we run into an issue:

```rust
   Compiling slice-example v0.1.0 (/home/dan/git/slice-example)
error[E0596]: cannot borrow `array` as mutable, as it is not declared as mutable
 --> src/main.rs:5:5
  |
5 |     array.resize_from(1);
  |     ^^^^^^^^^^^^^^^^^^^^ cannot borrow as mutable
  |
help: consider making the binding mutable
  |
3 | fn sort(mut array: &mut [u32]) {
  |         +++

For more information about this error, try `rustc --explain E0596`.
error: could not compile `slice-example` due to previous error
```

at `5ac70b231c0106399c556a394664e81def52d9be` we fix it... kinda

```
before: [
    3,
    5,
    7,
    9,
]
after: [
    3,
    5,
    7,
    9,
]
```

IIUC, we are not actually able to _resize_ the original `array: Vec` by _borrowing_ it's slice.

🤔 Curious behavior, but perhaps _safe_ behavior despite `resize_slice` using `unsafe` internally.
