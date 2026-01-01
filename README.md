# uy

Compile-time unit safety for Rust. Catches unit errors at compile time with zero runtime cost.

```rust
use uy::{Quantity, si};

let distance: Quantity<f32, si::m> = Quantity::new(100.0);
let time: Quantity<f32, si::s> = Quantity::new(9.58);
let speed: Quantity<f32, si::meters_per_second> = distance / time;

// This won't compile - can't add meters to seconds:
// let wrong = distance + time;
```

## Installation

```toml
[dependencies]
uy = "0.1"
```

## Usage

### Basic quantities

```rust
use uy::{Quantity, si};

let mass: Quantity<f64, si::kg> = Quantity::new(75.0);
let force: Quantity<f64, si::N> = Quantity::new(100.0);
```

### Unit algebra

Units combine through multiplication and division:

```rust
use uy::{Quantity, Mul, Div, si};

let length: Quantity<f32, si::m> = Quantity::new(5.0);
let width: Quantity<f32, si::m> = Quantity::new(3.0);
let area: Quantity<f32, Mul<si::m, si::m>> = length * width;

let time: Quantity<f32, si::s> = Quantity::new(2.0);
let velocity: Quantity<f32, Div<si::m, si::s>> = length / time;
```

### Prefix conversions

Scale is part of the type. Use `.convert()` to change scale:

```rust
use uy::{Quantity, si};

let meters: Quantity<i32, si::m> = Quantity::new(5);
let millimeters: Quantity<i32, si::milli<si::m>> = meters.convert();
assert_eq!(*millimeters, 5000);

let watts: Quantity<f64, si::W> = Quantity::new(1500.0);
let kilowatts: Quantity<f64, si::kilo<si::W>> = watts.convert();
assert_eq!(*kilowatts, 1.5);
```

### Accessing values

`Quantity` implements `Deref`, so use `*` to get the inner value:

```rust
use uy::{Quantity, si};

let temp: Quantity<f32, si::K> = Quantity::new(293.15);
println!("Temperature: {} K", *temp);
```

## SI Units

**Base units:** `s`, `m`, `kg`, `A`, `K`, `mol`, `cd`, `rad`, `unitless`

**Derived units:** `Hz`, `N`, `Pa`, `J`, `W`, `C`, `V`, `F`, `Ohm`, `S`, `Wb`, `T`, `H`, `Gy`, `Sv`, `Bq`, `lm`, `lx`, `kat`

**Compound units:** `meters_per_second`, `meters_per_second_squared`, `square_meters`, `cubic_meters`, `kilograms_per_cubic_meter`, `radians_per_second`, `newton_meters`, `volts_per_meter`, `watts_per_square_meter`, and more.

**Prefixes:** `quecto` through `quetta` (10⁻³⁰ to 10³⁰)

## License

MIT
