## Chapter 3 Fundamental Types

### Fixed-Width Numeric Types.

| Size         | Unsigned integer | Signed Integer | Floating Point |
|--------------|------------------|----------------|----------------|
| 8            | u8               | i8             | -              |
| 16           | u16              | i16            | -              |
| 32           | u32              | i32            | f32            |
| 64           | u64              | i64            | f64            |
| 128          | u128             | i128           | -              |
| Machine Word | usize            | isize          | -              |

*See* Code Examples

### Checked, Wrapping, Saturating and Overflowing Arithmetic

When an Integer operation overflows, Rust panic in a debug build, in a Release build it produces the Mathematically correct result modulo the range of the value, meaning that in no case Rust produces undefined behavior like C or C++

#### Example

```
let mut i = 1;
loop {
    i *=10; // overflow, panics in a debug build
}
```

```
let mut i: i32  = 1;
loop {
    // panic: multiplication overflow in any build
    i = i.checked_mul(10).expect("multiplication overflowed");
}
```

These integer arithmetic methods fall into 4 categories *see code examples*:

- Checked: The operations Returns an `Option` -> `Some(v)` if the mathematically correct result can be represented as a value of that type or `None` if it cannot.
- Wrapping: Operations returns the mathematically correct value modulo the range of the value.
- Saturating: The returned result is the closest one to the mathematically correct result
- Overflowing: The return is a tuple `(result, overflowed)` where `result` is the **wrapped** result, and `overflowed` is a `bool` that indicates if it overflowed or not.


### Floating-Point Types.

Every part of a floating point number after the integer part is optional, but at least one of the fractional part, exponent or type suffix must be present, `5.` is a valid floating-point number.
The types `f32` and `f64` have constants associated like `INFINITY` `NEG_INFINITY`, `NAN` `MIN` and `MAX`.

The `f32` and `f64` comes with a lot of methods for mathematical operations for example `2f64.sqrt()`.
The `std::f32::const` and `std::f64::const` comes with known constants like `PI` or `E`.

