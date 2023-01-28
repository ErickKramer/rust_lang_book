use float_eq::assert_float_eq;

fn main () {}

fn fahrenheit_celsius(temperature: f64) -> f64 {
    (temperature - 32.0) * 5.0/9.0
}

fn celsius_fahrenheit(temperature: f64) -> f64 {
    (temperature * 9.0/5.0) + 32.0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fahrenheit() {
        assert_eq!(fahrenheit_celsius(32.0), 0.0);
        assert_float_eq!(fahrenheit_celsius(15.0), -9.444, r2nd <= 0.000_1);
        assert_float_eq!(fahrenheit_celsius(50.0), 10.0, r2nd <= 0.000_1);
    }

    #[test]
    fn test_celsius() {
        assert_eq!(celsius_fahrenheit(0.0), 32.0);
        assert_float_eq!(celsius_fahrenheit(20.0), 68.0, r2nd <= 0.000_1);
        assert_float_eq!(celsius_fahrenheit(10.0), 50.0, r2nd <= 0.000_1);
    }
}

