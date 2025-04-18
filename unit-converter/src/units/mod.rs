pub mod length;
pub mod weight;
pub mod temperature;

pub use length::{Length, convert_length};
pub use weight::{Weight, convert_weight};
pub use temperature::{Temperature, convert_temperature};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_conversion() {
        let length = Length::new(1.0, "m");
        let converted = convert_length(length, "cm");
        assert_eq!(converted.value, 100.0);
        assert_eq!(converted.unit, "cm");
    }

    #[test]
    fn test_weight_conversion() {
        let weight = Weight::new(1.0, "kg");
        let converted = convert_weight(weight, "g");
        assert_eq!(converted.value, 1000.0);
        assert_eq!(converted.unit, "g");
    }

    #[test]
    fn test_temperature_conversion() {
        let temperature = Temperature::new(100.0, "C");
        let converted = convert_temperature(temperature, "F");
        assert_eq!(converted.value, 212.0);
        assert_eq!(converted.unit, "F");
    }
}

    #[test]
    fn test_invalid_length_conversion() {
        let length = Length::new(1.0, "m");
        let converted = convert_length(length, "invalid_unit");
        assert!(converted.is_err());
    }

    #[test]
    fn test_invalid_weight_conversion() {
        let weight = Weight::new(1.0, "kg");
        let converted = convert_weight(weight, "invalid_unit");
        assert!(converted.is_err());
    }

    #[test]
    fn test_invalid_temperature_conversion() {
        let temperature = Temperature::new(100.0, "C");
        let converted = convert_temperature(temperature, "invalid_unit");
        assert!(converted.is_err());
    }
