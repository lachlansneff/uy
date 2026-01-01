# uy - Type-Safe Units Library for Rust

## Project Overview

`uy` is a compile-time dimensional analysis library that encodes both units AND scale (power of ten) into Rust's type system. This prevents unit errors at compile time with clear error messages.

## Architecture

### Core Files

- **`src/lib.rs`** - Core library with `Quantity<T, U>`, `Unit` trait, `TenTo<N>`, and the `power_of_ten_unit_system!` macro
- **`src/si.rs`** - SI unit system: base units, prefixes, and derived units
- **`src/inner.rs`** - Internal type-level arithmetic bridging const generics to typenum

### Key Types

```rust
// Physical quantity with value type T and unit type U
pub struct Quantity<T, U: Unit> { val: T, _marker: PhantomData<U> }

// Power-of-ten multiplier (10^N)
pub struct TenTo<const N: i8>;

// SI unit type: Si<EXP, S, M, KG, A, K, MOL, CD, RAD>
// EXP = power-of-ten scale, rest are exponents for base units
pub type m = Si<0, 0, 1, 0, 0, 0, 0, 0, 0>;  // meter
pub type s = Si<0, 1, 0, 0, 0, 0, 0, 0, 0>;  // second
```

### Type-Level Unit Algebra

Units combine at compile time:
- `Mul<m, m>` = m^2 (area)
- `Div<m, s>` = m/s (velocity)
- `Mul<kg, Div<m, Mul<s, s>>>` = N (force)

Prefixes scale units:
- `milli<m>` = Si<-3, 0, 1, ...> (millimeters)
- `kilo<m>` = Si<3, 0, 1, ...> (kilometers)

### Conversions

Use `.convert()` to change scale:
```rust
let m: Quantity<i32, si::m> = Quantity::new(3);
let mm: Quantity<i32, si::milli<si::m>> = m.convert();
// mm = 3000
```

Conversion uses `MulPowerOfTen` trait which multiplies by 10^(target_exp - source_exp).

## SI Units

### Base Units
`s` (time), `m` (length), `kg` (mass), `A` (current), `K` (temperature), `mol` (amount), `cd` (luminosity), `rad` (angle), `unitless`

### Prefixes (10^N)
- Small: `quecto` (-30), `ronto` (-27), `yocto` (-24), `zepto` (-21), `atto` (-18), `femto` (-15), `pico` (-12), `nano` (-9), `micro` (-6), `milli` (-3), `centi` (-2), `deci` (-1)
- Large: `deka` (1), `hecto` (2), `kilo` (3), `mega` (6), `giga` (9), `tera` (12), `peta` (15), `exa` (18), `zetta` (21), `yotta` (24), `ronna` (27), `quetta` (30)

### Derived Units
`Hz`, `N`, `Pa`, `J`, `W`, `C`, `V`, `F`, `Ohm`, `S`, `Wb`, `T`, `H`, `Gy`

## Testing

Run tests with:
```bash
cargo test
```

Tests use inline `#[cfg(test)]` modules:
- `src/lib.rs` - Core Quantity and trait tests
- `src/si.rs` - SI-specific unit tests

Test coverage includes:
- Basic Quantity operations
- Arithmetic (add/sub/mul/div)
- Unit conversions
- SI prefixes
- Derived units
- Type algebra
- Physics scenarios

## Important Implementation Details

1. **Zero-cost abstraction**: `Quantity<T, U>` is `#[repr(transparent)]` with no runtime overhead
2. **Const generics + typenum**: Uses typenum for type-level arithmetic on unit exponents
3. **Range limit**: Exponents limited to -30 to +30 (matching SI prefix range)
4. **MulPowerOfTen semantics**: The trait multiplies by 10^(-exp), so positive exp divides and negative exp multiplies

## Build Commands

```bash
cargo build          # Build
cargo test           # Run tests
cargo doc --open     # Generate docs
```
