/**
 * The actual conversion part
 */

pub enum TempUnit {
    Celsius(f64),
    Fahrenheit(f64),
    Kelvin(f64),
}

use TempUnit::*;

#[derive(Debug)]
pub struct Temperature {
    celsius: f64,
    fahrenheit: f64,
    kelvin: f64,
}

impl From<TempUnit> for Temperature {
    fn from(temp: TempUnit) -> Self {
        match temp {
            Celsius(c) => Temperature {
                celsius: c,
                fahrenheit: c * (9.0/5.0) + 32.0,
                kelvin: c + 273.0,
            },
            Fahrenheit(f) => Temperature {
                celsius: (f + 32.0) / (9.0/5.0),
                fahrenheit: f,
                kelvin: (f + 32.0) / (9.0/5.0) + 273.0,
            },
            Kelvin(k) => Temperature {
                celsius: k - 273.0,
                fahrenheit: (k - 273.0) * (9.0/5.0) + 32.0,
                kelvin: k,
            },
        }
    }
}
