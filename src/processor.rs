use std::{
    io::{BufRead, Read, Seek, SeekFrom},
};

use crate::{
    accumulator::Accumulator, chunk::Chunk, errors::ProcessorError, line_parser::LineParser, partial_result::PartialResult
};

fn skip_partial_line<R: Read + Seek + BufRead>(reader: &mut R, mut pos: u64  ) -> Result<u64, ProcessorError> {
    if pos != 0 {
        let mut discard = Vec::new();
        reader.read_until(b'\n', &mut discard)?;
        pos += discard.len() as u64;
    }
    Ok(pos) 
 }

/// Reads and processes the chunk, returning a PartialResult.
pub fn process_chunk<R: Read + Seek + BufRead>(
    reader: &mut R,
    chunk: Chunk
) -> Result<PartialResult, ProcessorError> {
    reader.seek(SeekFrom::Start(chunk.start)).map_err(ProcessorError::Io)?;
    let mut result = PartialResult::new();
    let mut pos = chunk.start;
    pos = skip_partial_line(reader, pos)?;
    let mut line = Vec::new();

    while pos < chunk.end {
        line.clear();
        let bytes_read = reader
            .read_until(b'\n', &mut line)
            .map_err(ProcessorError::Io)?;
        if bytes_read == 0 {
            break;
        }

        pos += bytes_read as u64;


        let (year, month, day, no2, lat, lon) =
            match parse_line(&line) {
                Some(v) => v,
                None => continue,
        };


        // T1 - acumular por (year, month, day)
        accummulate_per_y_m_d(&mut result, year, month, day, no2);

        // T2 - acumular por (year, grid_lat, grid_lon)

        accumulate_per_y_g_lat_g_lon(&mut result, year, lat, lon, no2);
        
    }

    Ok(result)
}

    fn accummulate_per_y_m_d(result: &mut PartialResult, year: u16, month: u8, day: u8, no2: f64) {
        result
            .t1
            .entry((year, month, day))
            .or_insert(Accumulator::new())
            .add(no2);
    }

    fn accumulate_per_y_g_lat_g_lon(result: &mut PartialResult, year: u16, lat: f64, lon: f64, no2: f64) {
        let grid_lat = (lat / 0.5).floor() as i32;
        let grid_lon = (lon / 0.5).floor() as i32;
        result
            .t2
            .entry((year, grid_lat, grid_lon))
            .or_insert(Accumulator::new())
            .add(no2);
    }

    fn parse_line(line: &[u8]) -> Option<(u16, u8, u8, f64, f64, f64)> {
        let parser = match LineParser::new(&line) {
            Some(p) => p,
            None => return None,
        };

        let (year, month, day, no2 , lat, lon) =
            match (parser.year(), parser.month(), parser.day(), parser.no2(), parser.lat(), parser.lon()) {
                (Some(y), Some(mo), Some(d), Some(n), Some(la), Some(lo)) => (y, mo, d, n, la, lo),
                _ => return None,
            };

        Some((year, month, day, no2, lat, lon))
    }

#[cfg(test)]  
mod tests {     
    use super::*;
    #[test]
    fn test_process_chunk_parses_valid_lines() -> Result<(), Box<dyn std::error::Error>> {
        let data = b"
    x,1,2,10,ignored,2024,1,1,x
    x,3,4,20,ignored,2024,1,1,x
    ";

        let mut reader = std::io::Cursor::new(&data[..]);
        let chunk = Chunk { start: 0, end: data.len() as u64 };

        let result = process_chunk(&mut reader, chunk)?;

        let key = (2024, 1, 1);

        let acc = result.t1.get(&key).unwrap();

        assert_eq!(acc.count, 2);
        assert_eq!(acc.sum, 30.0);

        Ok(())
    }

    #[test]
    fn test_process_chunk_skips_invalid_lines() -> Result<(), Box<dyn std::error::Error>> {
        let data = b"
    invalid_line
    x,1,2,10,ignored,2024,1,1,x
    bad_data
    ";

        let mut reader = std::io::Cursor::new(&data[..]);
        let chunk = Chunk { start: 0, end: data.len() as u64 };

        let result = process_chunk(&mut reader, chunk)?;

        assert_eq!(result.t1.len(), 1);

        Ok(())
    }

    #[test]
    fn test_process_chunk_groups_by_date() -> Result<(), Box<dyn std::error::Error>> {
        let data = b"
    x,0,0,10,ignored,2024,1,1,x
    x,0,0,20,ignored,2024,1,2,x
    x,0,0,30,ignored,2024,1,2,x
    ";

        let mut reader = std::io::Cursor::new(&data[..]);
        let chunk = Chunk { start: 0, end: data.len() as u64 };

        let result = process_chunk(&mut reader, chunk)?;

        assert_eq!(result.t1.len(), 2);

        assert_eq!(result.t1.get(&(2024, 1, 1)).unwrap().count, 1);
        assert_eq!(result.t1.get(&(2024, 1, 2)).unwrap().count, 2);

        Ok(())
    }    

    #[test]
    fn test_process_chunk_grid_aggregation() -> Result<(), Box<dyn std::error::Error>> {
        let data = b"
    x,0.0,0.0,10,ignored,2024,1,1,x
    x,0.4,0.4,20,ignored,2024,1,1,x
    x,0.6,0.6,30,ignored,2024,1,1,x
    ";

        let mut reader = std::io::Cursor::new(&data[..]);
        let chunk = Chunk { start: 0, end: data.len() as u64 };

        let result = process_chunk(&mut reader, chunk)?;

        assert!(result.t2.len() >= 1);

        let total: f64 = result.t2.values().map(|a| a.sum).sum();

        assert_eq!(total, 60.0);

        Ok(())
    }

    #[test]
    fn test_process_chunk_skips_partial_start_line() -> Result<(), Box<dyn std::error::Error>> {
        let data = b"
    x,0,0,10,ignored,2024,1,1,x
    x,0,0,20,ignored,2024,1,1,x
    x,0,0,30,ignored,2024,1,1,x
    ";

        let mut reader = std::io::Cursor::new(&data[..]);

        // arrancar en medio de la primera línea
        let chunk = Chunk { start: 5, end: data.len() as u64 };

        let result = process_chunk(&mut reader, chunk)?;

        // debe procesar solo líneas completas
        assert!(result.t1.len() > 0);

        Ok(())
    }   

    #[test]
    fn test_process_chunk_empty_range() -> Result<(), Box<dyn std::error::Error>> {
        let data = b"x,0,0,10,ignored,2024,1,1,x\n";

        let mut reader = std::io::Cursor::new(&data[..]);

        let chunk = Chunk { start: 0, end: 0 };

        let result = process_chunk(&mut reader, chunk)?;

        assert!(result.t1.is_empty());

        Ok(())
    }
}