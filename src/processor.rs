use std::{
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
    path::Path,
};

use crate::{
    chunk::Chunk,
    line_parser::LineParser,
    partial_result::{Accumulator, PartialResult},
};

/// Processes a single chunk of the CSV file.
pub struct Processor {
    path: std::path::PathBuf,
    chunk: Chunk,
}

impl Processor {
    /// Creates a new Processor for the given path and chunk.
    pub fn new(path: &Path, chunk: Chunk) -> Self {
        Self {
            path: path.to_path_buf(),
            chunk,
        }
    }

    /// Reads and processes the chunk, returning a PartialResult.
    pub fn process_chunk(self) -> PartialResult {
        let mut file = File::open(&self.path).unwrap();
        file.seek(SeekFrom::Start(self.chunk.start)).unwrap();

        let mut reader = BufReader::new(file);
        let mut result = PartialResult::new();
        let mut pos = self.chunk.start;
        let mut line = Vec::new();

        while pos < self.chunk.end {
            line.clear();
            let bytes_read = reader.read_until(b'\n', &mut line).unwrap();
            if bytes_read == 0 { break; }
            pos += bytes_read as u64;

            let parser = match LineParser::new(&line) {
                Some(p) => p,
                None => continue,
            };

            if !parser.is_valid_quality() { continue; }

            let (year, month, day, no2) = match (
                parser.year(),
                parser.month(),
                parser.day(),
                parser.no2(),
            ) {
                (Some(y), Some(mo), Some(d), Some(n)) => (y, mo, d, n),
                _ => continue,
            };

            // T1 - acumular por (year, month, day)
            result.t1
                .entry((year, month, day))
                .or_insert(Accumulator { sum: 0.0, count: 0 })
                .merge(&Accumulator::new(no2));

            // T2 - acumular por (year, grid_lat, grid_lon)
            if let (Some(lat), Some(lon)) = (parser.lat(), parser.lon()) {
                let grid_lat = (lat / 0.5).floor() as i32;
                let grid_lon = (lon / 0.5).floor() as i32;
                result.t2
                    .entry((year, grid_lat, grid_lon))
                    .or_insert(Accumulator { sum: 0.0, count: 0 })
                    .merge(&Accumulator::new(no2));
            }
        }

        result
    }
}