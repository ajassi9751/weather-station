use crate::io::csvmanager::MatchIntoType;

enum Data {
    Tempurature(f64),
    Humidity(f64),
    WindSpeed(f64),
}

impl MatchIntoType for Data {
    fn match_into_type(self) -> (String, usize) {
        match self {
            Self::Tempurature(v) => (format!("{}", v), 0),
            Self::Humidity(v) => (format!("{}", v), 0),
            Self::WindSpeed(v) => (format!("{}", v), 0),
        }
    }
}
