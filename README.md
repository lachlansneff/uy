# uy

Compile-time unit safety for Rust. Catches unit errors at compile time with zero runtime cost.

See [docs.rs/uy](https://docs.rs/uy) for the full API.

```rust
use uy::{Quantity, si};

let distance: si::meters<f32> = Quantity::new(100.0);
let time: si::seconds<f32> = Quantity::new(9.58);
let speed: si::meters_per_second<f32> = distance / time;

// This won't compile - can't add meters to seconds:
// let wrong = distance + time;
```

## Installation

```toml
[dependencies]
uy = "0.2"
```

## Usage

### Basic quantities

```rust
use uy::{Quantity, si};

let mass: si::kilograms<f64> = Quantity::new(75.0);
let force: si::newtons<f64> = Quantity::new(100.0);
```

### Unit algebra

Units combine through multiplication and division. All derived and compound units are just type aliases to combinations of `Mul`, `Div`, and base units:

```rust
use uy::{Quantity, Mul, Div, si};

// These are equivalent:
let v1: si::meters_per_second<f32> = Quantity::new(10.0);
let v2: Quantity<f32, Div<si::units::m, si::units::s>> = Quantity::new(10.0);

// Unit algebra works automatically:
let length: si::meters<f32> = Quantity::new(5.0);
let width: si::meters<f32> = Quantity::new(3.0);
let area: si::square_meters<f32> = length * width;  // m * m = m²
```

### Prefix conversions

Scale is encoded in the type via `TenTo<N>`. Prefixes like `milli` and `kilo` are type aliases:

```rust
use uy::{Quantity, si};

// si::milli<U> is just Mul<U, TenTo<-3>>
let meters: si::meters<i32> = Quantity::new(5);
let millimeters: Quantity<i32, si::milli<si::units::m>> = meters.convert();
assert_eq!(*millimeters, 5000);
```

### Accessing values

`Quantity` implements `Deref`, so use `*` to get the inner value:

```rust
use uy::{Quantity, si};

let temp: si::kelvin<f32> = Quantity::new(293.15);
println!("Temperature: {} K", *temp);
```

## Module structure

- `si` - Quantity type aliases (`si::meters<T>`, `si::newtons<T>`, etc.)
- `si::units` - Raw unit types for use with `Quantity<T, U>` and prefixes
- `si::prefixes` - Scale prefixes (`milli`, `kilo`, etc.)

## License

MIT
