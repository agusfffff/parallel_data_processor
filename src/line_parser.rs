/// Parses a single CSV line as a byte slice, providing lazy
/// access to each field without intermediate allocations.
pub struct LineParser<'a> {
    fields: [&'a [u8]; 9],
}

impl<'a> LineParser<'a> {
    /// Creates a new LineParser from a line of bytes.
    /// Returns None if the line does not have enough fields.
    pub fn new(line: &'a [u8]) -> Option<Self> {
        let mut iter = line.splitn(9, |&b| b == b',');
        let fields = std::array::from_fn(|_| iter.next().unwrap_or(&[]));
        Some(Self { fields })
    }

    /// Returns the year field.
    pub fn year(&self) -> Option<u16> {
        parse_u16(self.fields[5])
    }

    /// Returns the month field.
    pub fn month(&self) -> Option<u8> {
        parse_u8(self.fields[6])
    }

    /// Returns the day field.
    pub fn day(&self) -> Option<u8> {
        parse_u8(self.fields[7])
    }

    /// Returns the latitude field.
    pub fn lat(&self) -> Option<f64> {
        parse_f64(self.fields[1])
    }

    /// Returns the longitude field.
    pub fn lon(&self) -> Option<f64> {
        parse_f64(self.fields[2])
    }

    /// Returns the NO2 column value.
    pub fn no2(&self) -> Option<f64> {
        parse_f64(self.fields[3])
    }

}

fn parse_f64(bytes: &[u8]) -> Option<f64> {
    std::str::from_utf8(bytes).ok()?.trim().parse().ok()
}

fn parse_u16(bytes: &[u8]) -> Option<u16> {
    std::str::from_utf8(bytes).ok()?.trim().parse().ok()
}

fn parse_u8(bytes: &[u8]) -> Option<u8> {
    std::str::from_utf8(bytes).ok()?.trim().parse().ok()
}


#[cfg(test)] 
mod tests {    
    use super::*;

    #[test]
    fn test_line_parser_valid_line() {
        // lat=1, lon=2, no2=10, year=2024, month=1, day=1
        let line = b"x,1,2,10,ignored,2024,1,1,extra";

        let parser = LineParser::new(line).unwrap();

        assert_eq!(parser.year(), Some(2024));
        assert_eq!(parser.month(), Some(1));
        assert_eq!(parser.day(), Some(1));

        assert_eq!(parser.lat(), Some(1.0));
        assert_eq!(parser.lon(), Some(2.0));
        assert_eq!(parser.no2(), Some(10.0));
    }

    #[test]
    fn test_line_parser_missing_fields() {
        let line = b"only,three,fields";

        let parser = LineParser::new(line).unwrap();

        // debería devolver None en todo
        assert!(parser.year().is_none());
        assert!(parser.month().is_none());
        assert!(parser.day().is_none());
        assert!(parser.lat().is_none());
        assert!(parser.lon().is_none());
        assert!(parser.no2().is_none());
    }

    #[test]
    fn test_line_parser_with_spaces() {
        let line = b"x, 1 , 2 , 10 ,x, 2024 , 3 , 15 ,x";

        let parser = LineParser::new(line).unwrap();

        assert_eq!(parser.year(), Some(2024));
        assert_eq!(parser.month(), Some(3));
        assert_eq!(parser.day(), Some(15));

        assert_eq!(parser.lat(), Some(1.0));
        assert_eq!(parser.lon(), Some(2.0));
        assert_eq!(parser.no2(), Some(10.0));
    }

    #[test]
    fn test_line_parser_invalid_numbers() {
        let line = b"x,abc,def,not_a_number,x,2024,1,1,x";

        let parser = LineParser::new(line).unwrap();

        assert!(parser.lat().is_none());
        assert!(parser.lon().is_none());
        assert!(parser.no2().is_none());

        assert_eq!(parser.year(), Some(2024));
    }

}