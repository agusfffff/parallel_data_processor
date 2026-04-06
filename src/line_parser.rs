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
    pub fn year(&self) -> Option<u16> { parse_u16(self.fields[5]) }

    /// Returns the month field.
    pub fn month(&self) -> Option<u8> { parse_u8(self.fields[6]) }

    /// Returns the day field.
    pub fn day(&self) -> Option<u8> { parse_u8(self.fields[7]) }

    /// Returns the latitude field.
    pub fn lat(&self) -> Option<f64> { parse_f64(self.fields[1]) }

    /// Returns the longitude field.
    pub fn lon(&self) -> Option<f64> { parse_f64(self.fields[2]) }

    /// Returns the NO2 column value.
    pub fn no2(&self) -> Option<f64> { parse_f64(self.fields[3]) }

    /// Returns the quality flag.
    pub fn quality(&self) -> Option<f64> { parse_f64(self.fields[4]) }

    /// Returns true if the quality flag is above 0.5.
    pub fn is_valid_quality(&self) -> bool {
        self.quality().map_or(false, |q| q > 0.5)
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