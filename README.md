# uy

A typesafe, flexible, simple, and user-friendly unit system library for Rust that has good error messages.

## Usage

`uy` not only stores the unit of a value in the typesystem, it also stores the scale within the unit's type itself.

For example, `Quantity<f32, si::m>` is not the same type as `Quantity<f32, si::kilo<si::m>>`.

To convert between types like that, call the `.convert()` method on `Quantity`.

## Example

```rust
use uy::{Quantity, si};

fn how_long(d: Quantity<f32, si::m>, v: Quantity<f32, uy::Div<si::m, si::s>>) -> Quantity<f32, si::s> {
    d / v
}
```
