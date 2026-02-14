use crate::io::csvmanager::MatchIntoType;

enum Data {
    Tempurature,
    Humidity,
    WindSpeed,
}

impl MatchIntoType for Data {
    fn match_into_type(self) -> (String, usize) {
        (String::from(""), 0)
    }
}
