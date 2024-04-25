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
The types `f32` and `f64` have constants associated like `INFINITY` `NEG_INFINITY`, `NAN`, `MIN` and `MAX`.

The `f32` and `f64` comes with a lot of methods for mathematical operations for example `2f64.sqrt()`.
The `std::f32::const` and `std::f64::const` comes with known constants like `PI` or `E`.

### The bool type 

Rust is very strict, control structures require that their conditions are `bool` expressions.

```
if x != 0 { ...  }
```
not simply

```
if x { ...  }
```

Ruts's `as` operator can convert `bool` values to integer types.

```
assert_eq!(false as i32, 0);
assert_eq!(true as i32, 1);
```

**But** it won't convert in the other direction

Although `bool` only needs 1 bit, Rust will use an entire byte so you can create a pointer to it.

### Characters

Rust's type `char` represents a single Unicode character, as a 32 bit value.

Rust uses the `char` type for single characters, but uses UTF-8 enconding for strings and streams of text

Character literals are characters enclosed in single quotes like '8' or '!', you can also use the full breadth of Unicode

You need to excape certain special characters:

| Character       | Rust character literal |
|-----------------|------------------------|
| Single quote    | '\''                   |
| Backslash       | '\\'                   |
| Newline         | '\n'                   |
| Carriage Return | '\r'                   |
| Tab             | '\t'                   |

You can write a character's unicode code point in hexadecimal if you prefer:

* If the character is in the range U+0000 to U+007F (The ASCII character set) you can write '\xHH' where HH is a 2 digit hexadecimal number.
* You could also write '\u{HHHHHH}' where HHHHHH is a six-digit long hexadecimal number.
* Rust never converts implicitly between `char` and any other type.
* You can use `as` to convert to integer type.

The `u8` is the **only** type the operator `as` will convert to `char`, types other than `u8` will include values that are not permitted as a char.

The standard library `std:char::from_u32` takes any `u32` value and returns an `Option<char>`: if the `u32` is not an allowed Unicode code point then `from_u32` returns `None`, otherwise it returns `Some(c)` where `c` is the `char` result.

### Tuples

A Tuple is a pair, a tripe, a quadruple... etc.
You can write a tuple a sequence of elements separated by commans and surrounded by parenthesis, for example `("Argentina", 1980)`.
Given a tuple `t` you can access the tuple by `t.1` `t.2` and so on.
You **cannot** access tuple's element by doing `t.i` or `t[i]`
Rust sometimes uses tuples to return multiple values from a function, for example:
```
fn split_at(&self, mid: usize) -> (&str, &str)
```
The above returns a tuple of two `&str` (string slices)

*See the example on how this function is used*

The other commonly used tuple type is the zero-tuple `()` traditionally called the `unit type`

### Pointer Types

Rust has several types that represents memory addresses.

* References
* Boxes
* Raw Pointers


#### References

A value of `&String` (pronounced 'ref String') is a reference to a `String` value.
The expression `&x` produces a reference to `x` in Rust terminology we say that "it borrows a reference to x".
Given a reference `r` `*r` is the value `r` points to (just like C)
Unlike C pointers, Rust references are **never** null, there's no way to produce a null reference in safe Rust.
Rust tracks ownership and lifetime of values, things like dangling pointers are catched at compile time.

Rust references comes in 2 flavors:

* `&T` --> an inmmutable shared reference, you can have many shared references to a given value but they're **read only**, modifying the value they point to is forbidden, it's like `const T*` in C.
* `&mut T` --> a mutable **exclusive** reference, you can read and modify the value it points to, but for **as long as the reference exists** you cannot have another reference of any kind to that value.

The above is called "multiple readers **or** single writers" you cannot have both

#### Boxes

Is what you can use to allocate a value in the Heap.
This is accomplished by doing `Box::new`

```
let t = (12, "eggs");
let b = Box::new(t); // allocate a tuple in the heap
```

The type of `t` is `(i32, &str)` so the value of `b` is `Box<(i32, &str)`
When `b` goes out of scope the memory allocated is freed immediately **unless `b` has been moved**


#### Raw Pointers 

Raw pointers are just like regular pointers in C, using a raw pointer is unsafe, because **Rust makes no effort to track what it points to** 
You may only dereference raw pointer inside an `unsafe block`

### Array, Vectors and Slices

Rust had 3 types for representing a sequence of values in memory

- The type `[T; N]`: This is an `array` of `N` values of type `T`, the array size is a constant determined at compile time, you **cannot** add new elements or shrink an array.
- The type `Vec<T` called a `vector of T's`: Is **dynamically** allocated, it's a growable sequence of elements of type `T`, the vector's elements live **on the heap**.
- The types `&[T]` and `&mut[T]` called a **shared slice of T's** and **mutable slice of T's**: These are reference to a series of elements that live somewhere else, it's like a pointer to its first element
  - A mutable slice `&mut[T]` can be read and modify its elements **but i cannot be shared**
  - A shared slice `&[T]` can be shared among several readers, but it **cannot be modified**

Given a value `v` of any of these 3 types, the expression `v.len()` gives the number of elements, `v[i]` refers to the `ith` element.
`i` **must** be of `usize`, you cannot use any other type as an index.

### Arrays

The simplest way to write an array is within brackets:

```
let lazy_caterer: [u32; 6] = [1, 2, 3, 4, 5, 6];
let taxonomy = ["Animalia", "Arthropoda", "Insecta"];
```

You'll see this sintax for fixed-length buffers: `[0u8, 1024]` can be a one kilobyte buffer filled with zeroes

An array's length is part of its type and fixed at compile time, if `n` is variable you **cannot** do `[true; n]` to get an array of `n` elements.

If you need an array whose length varies **don't use an array use a vector instead**.

The usual methods that you'd see on arrays -iterating, searching, sorting, etc- are all methods provided on `slices` not on `arrays` but Rust implicitly converts a reference to an array into a slice

```
let mut chaos = [3, 5, 4, 1, 2];
chaos.sort();
assert_eq!(chaos, [1, 2, 3, 4, 5]);
```
