extern crate rand;

#[cfg(test)]
mod tests {
    // Resolve collisions with internal ::rand
    use super::rand;
    use super::rand::Rng;

    use temperature::Temperature;
    use temperature::TempUnit::*;

    #[test]
    fn from_celsius_test() {
        let c: f64 = rand::thread_rng().gen::<f64>();

        let expected = Temperature {
            celsius: c,
            fahrenheit: c * (9.0 / 5.0) + 32.0,
            kelvin: c + 273.0,
        };

        let from_celsius = Temperature::from(Celsius(c));

        assert_eq!(
            expected,
            from_celsius,
            "\nStarted with: {}\nExpected:\n{:#?}\nGot:\n{:#?}",
            c,
            expected,
            from_celsius
        );
    }

    #[test]
    fn from_fahrenheit_test() {
        let f: f64 = rand::thread_rng().gen::<f64>();

        let expected = Temperature {
            celsius: (f - 32.0) / (9.0 / 5.0),
            fahrenheit: f,
            kelvin: (f - 32.0) / (9.0 / 5.0) + 273.0,
        };

        let from_fahrenheit = Temperature::from(Fahrenheit(f));

        assert_eq!(
            expected,
            from_fahrenheit,
            "\nStarted with: {}\nExpected:\n{:#?}\nGot:\n{:#?}",
            f,
            expected,
            from_fahrenheit
        );
    }

    #[test]
    fn from_kelvin_test() {
        let k: f64 = rand::thread_rng().gen::<f64>();

        let expected = Temperature {
            celsius: k - 273.0,
            fahrenheit: (k - 273.0) * (9.0 / 5.0) + 32.0,
            kelvin: k,
        };

        let from_kelvin = Temperature::from(Kelvin(k));

        assert_eq!(
            expected,
            from_kelvin,
            "\nStarted with: {}\nExpected:\n{:#?}\nGot:\n{:#?}",
            k,
            expected,
            from_kelvin
        );
    }
}
