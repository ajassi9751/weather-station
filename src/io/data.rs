use crate::io::csvmanager::MatchIntoType;

pub enum Data {
    Tempurature(f64),
    Humidity(f64),
    AirPressure(f64),
    WindSpeed(f64)
}

// Consumes data to give to csvmanager
impl MatchIntoType for Data {
    fn match_into_type(self) -> (String, usize) {
        match self {
            Self::Tempurature(v) => (v.to_string(), 0),
            Self::Humidity(v) => (v.to_string(), 1),
            Self::AirPressure(v) => (v.to_string(), 2),
            Self::WindSpeed(v) => (v.to_string(), 3),
        }
    }
}
