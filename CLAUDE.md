# uy - Type-Safe Units Library for Rust

## Project Overview

`uy` is a compile-time dimensional analysis library that encodes both units AND scale (power of ten) into Rust's type system. This prevents unit errors at compile time with clear error messages.

## Architecture

### Core Files

- **`src/lib.rs`** - Core library with `Unit` trait, `TenTo<N>`, `Mul`/`Div` combinators, and the `power_of_ten_unit_system!` macro
- **`src/quantity.rs`** - The `Quantity<T, U>` wrapper type
- **`src/si.rs`** - SI unit system: base units, prefixes, derived units, and quantity type aliases
- **`src/inner.rs`** - Internal type-level arithmetic bridging const generics to typenum

### Module Structure

```
si/
├── units/           # Raw unit types (m, s, kg, N, Hz, meters_per_second, etc.)
│   └── derived/     # Derived unit types
├── prefixes/        # Scale prefixes (milli, kilo, etc.)
└── (top-level)      # Quantity type aliases (meters<T>, seconds<T>, newtons<T>, etc.)
```

**Important**: The `si` module has two layers:
- `si::units::*` - Raw unit types for use with `Quantity<T, U>` and prefixes
- `si::*` (top-level) - Quantity type aliases like `si::meters<T>` = `Quantity<T, units::m>`

### Key Design Pattern

All derived and compound units are just type aliases to `Mul`/`Div` combinations. Prefixes are `Mul<U, TenTo<N>>`. This means the system is extensible without special cases.

```rust
pub type Hz = Div<unitless, s>;
pub type N = Mul<kg, Div<m, Mul<s, s>>>;
pub type milli<U> = Mul<U, TenTo<-3>>;
```

## Guidelines

### Documentation

- **Describe patterns and structure, not exhaustive lists.** Enumerating every unit, prefix, or variant creates maintenance burden and goes stale. Instead, explain the underlying system so users can discover specifics via IDE or `cargo doc`.
- Keep architecture documentation in sync when refactoring module structure.

### Rust Type System

- **Type aliases only exist in the type namespace.** A `type Foo = Bar;` cannot be used as a value, even if `Bar` is a unit struct. This is a fundamental Rust limitation, not a bug.
- When designing APIs, consider whether users need type-level constructs (for generic bounds) or value-level constructs (for expressions).

### Testing

- Use inline `#[cfg(test)]` modules, not separate test files.
- **Test the underlying mechanics, not every instance.** If the math/logic is tested once, don't redundantly test every combination that uses it.
- Prefer testing behavior over testing specific types.

### Code Style

- `#[repr(transparent)]` for zero-cost wrappers
- Const generics bridged to typenum for type-level arithmetic
- Exponent range: -30 to +30 (SI prefix range)

## Build Commands

```bash
cargo build          # Build
cargo test           # Run tests
cargo doc --open     # Generate docs
```
