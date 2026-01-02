# uy

Compile-time unit safety for Rust. Catches unit errors at compile time with zero runtime cost.

See [docs.rs/uy](https://docs.rs/uy) for the full API.

```rust
use uy::{Quantity, si};

let distance: Quantity<f32, si::m> = Quantity::new(100.0);
let time: Quantity<f32, si::s> = Quantity::new(9.58);
let speed: Quantity<f32, si::meter_per_second> = distance / time;

// This won't compile - can't add meters to seconds:
// let wrong = distance + time;
```

## Installation

```toml
[dependencies]
uy = "0.3"
```

## Usage

### Basic quantities

```rust
use uy::{Quantity, si};

let mass: Quantity<f64, si::kg> = Quantity::new(75.0);
let force: Quantity<f64, si::N> = Quantity::new(100.0);
```

### Unit algebra

Units combine through multiplication and division. All derived and compound units are just type aliases to combinations of `Mul`, `Div`, and base units:

```rust
use uy::{Quantity, Mul, Div, si};

// These are equivalent:
let v1: Quantity<f32, si::meter_per_second> = Quantity::new(10.0);
let v2: Quantity<f32, Div<si::m, si::s>> = Quantity::new(10.0);

// Unit algebra works automatically:
let length: Quantity<f32, si::m> = Quantity::new(5.0);
let width: Quantity<f32, si::m> = Quantity::new(3.0);
let area: Quantity<f32, si::square_meter> = length * width;  // m * m = m²
```

### Prefix conversions

Scale is encoded in the type via `TenTo<N>`. Prefixes like `milli` and `kilo` are type aliases:

```rust
use uy::{Quantity, si};

// si::milli<U> is just Mul<U, TenTo<-3>>
let meters: Quantity<i32, si::m> = Quantity::new(5);
let millimeters: Quantity<i32, si::milli<si::m>> = meters.convert();
assert_eq!(*millimeters, 5000);
```

### Accessing values

`Quantity` implements `Deref`, so use `*` to get the inner value:

```rust
use uy::{Quantity, si};

let temp: Quantity<f32, si::K> = Quantity::new(293.15);
println!("Temperature: {} K", *temp);
```

## `no_std` Support

This library supports compiling in `no_std` environments with nightly rust using the
`core_float_math` feature. This feature is required to perform `powi` operations without
`std`.

To use this library without `std`, disable the default-features when depending on it. The
`std` feature is enabled by default, which disables `no_std` support.

## License

MIT
