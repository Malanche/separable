# Trait to separate Enums

A trait that helps split up an vector of enums into a tuple with a vector per variant.
Please let me know if such a functionality can be achieved in Rust without the macro.

```rust
use separable::Separable;

#[derive(Separable)]
enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
    Kelvin(f64)
}

fn main() {
    // A bunch of measurements...
    let measurements = vec![
        Temperature::Celsius(23.0),
        Temperature::Fahrenheit(2.0),
        Temperature::Celsius(22.5),
        Temperature::Kelvin(288.0),
        Temperature::Celsius(23.1),
        Temperature::Fahrenheit(5.0)
    ];
    
    // We separate all the variants, in order
    let (celsius, fahrenheit, kelvin) = measurements.into_iter().collect();
    
    // Quick verification
    assert_eq!(celsius, vec![23.0f64, 22.5f64, 23.1f64]);
    assert_eq!(fahrenheit, vec![2.0f64, 5.0f64]);
    assert_eq!(kelvin, vec![288.0f64]);
}
```