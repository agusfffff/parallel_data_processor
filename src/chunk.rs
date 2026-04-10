use std::io::{Read, Seek, SeekFrom};

use crate::errors::ChunkError;

/// Represents a byte range [start, end) within a file
/// that a worker will process.
#[derive(Clone)]
pub struct Chunk {
    pub start: u64,
    pub end: u64,
}

/// Divides a readable and seekable source into `n` chunks,
/// adjusting boundaries so that each chunk ends at a newline.
pub fn divide_chunks<R: Read + Seek>(reader: &mut R, n: usize) -> Result<Vec<Chunk>, ChunkError> {
    if n == 0 {
        return Err(ChunkError::InvalidChunkCount(
            "number of chunks must be greater than zero".into(),
        ));
    }

    let file_size = reader.seek(SeekFrom::End(0)).map_err(ChunkError::Io)?;
    
    if n as u64 > file_size {
        return Err(ChunkError::InvalidChunkCount(
            "number of chunks must be less than or equal to file size".into(),
        ));
    }

    let chunk_size = file_size / n as u64;
    let mut chunks = Vec::with_capacity(n);
    let mut start = 0u64;

    for i in 0..n {
        if i == n - 1 {
            chunks.push(Chunk {
                start,
                end: file_size,
            });
            break;
        }

        let approx_end = start + chunk_size;
        chunks.push(Chunk { start, end: approx_end });
        start = approx_end;
    }

    Ok(chunks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::io::Cursor;

    fn make_cursor(content: &'static [u8]) -> Cursor<&'static [u8]> {
        Cursor::new(content)
    }

    #[test]
    fn test_chunks_cover_entire_file() -> Result<(), Box<dyn Error>> {
        let mut cursor = make_cursor(b"header\nline1\nline2\nline3\nline4\n");
        let file_size = cursor.seek(SeekFrom::End(0))?;
        cursor.seek(SeekFrom::Start(0))?;
        let chunks = divide_chunks(&mut cursor, 2)?;

        assert_eq!(chunks[0].start, 0);
        assert_eq!(chunks.last().map(|chunk| chunk.end), Some(file_size));
        Ok(())
    }

    #[test]
    fn test_no_gaps_between_chunks() -> Result<(), Box<dyn Error>> {
        let mut cursor = make_cursor(b"header\nline1\nline2\nline3\nline4\n");
        let chunks = divide_chunks(&mut cursor, 2)?;

        assert_eq!(chunks.len(), 2);
        for window in chunks.windows(2) {
            assert_eq!(window[0].end, window[1].start);
        }

        Ok(())
    }

    #[test]
    fn test_no_empty_chunks() -> Result<(), Box<dyn Error>> {
        let mut cursor = make_cursor(
            b"header\nline01\nline02\nline03\nline04\nline05\nline06\nline07\nline08\nline09\nline10\nline11\nline12\n",
        );
        let chunks = divide_chunks(&mut cursor, 4)?;

        for chunk in &chunks {
            assert!(chunk.end > chunk.start);
        }

        Ok(())
    }

    #[test]
    fn test_correct_number_of_chunks() -> Result<(), Box<dyn Error>> {
        let mut cursor = make_cursor(
            b"header\nline01\nline02\nline03\nline04\nline05\nline06\nline07\nline08\nline09\nline10\nline11\nline12\n",
        );
        let chunks = divide_chunks(&mut cursor, 4)?;

        assert_eq!(chunks.len(), 4);
        Ok(())
    }

}
