
#[derive(Debug, Clone, Copy)]
pub enum Length {
    Meters(f64),
    Kilometers(f64),
    Miles(f64),
    Feet(f64),
    Inches(f64),
}

impl Length {
    pub fn to_meters(&self) -> f64 {
        match self {
            Length::Meters(value) => *value,
            Length::Kilometers(value) => value * 1000.0,
            Length::Miles(value) => value * 1609.344,
            Length::Feet(value) => value * 0.3048,
            Length::Inches(value) => value * 0.0254,
        }
    }
}

pub fn convert_length(from: Length, to_unit: &str) -> Result<f64, String> {
    let meters = from.to_meters();
    
    match to_unit.to_lowercase().as_str() {
        "m" => Ok(meters),
        "km" => Ok(meters / 1000.0),
        "mi" => Ok(meters / 1609.344),
        "ft" => Ok(meters / 0.3048),
        "in" => Ok(meters / 0.0254),
        _ => Err(String::from("Invalid unit. Supported units: m, km, mi, ft, in"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meters_to_kilometers() {
        let length = Length::Meters(1000.0);
        let result = convert_length(length, "km").unwrap();
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_kilometers_to_miles() {
        let length = Length::Kilometers(1.0);
        let result = convert_length(length, "mi").unwrap();
        assert!((result - 0.621371).abs() < 0.0001);
    }
}
