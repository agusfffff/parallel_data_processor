use std::io::{Read, Seek, SeekFrom};

/// Represents a byte range [start, end) within a file
/// that a worker will process.
pub struct Chunk {
    pub start: u64,
    pub end: u64,
}

/// Divides a readable and seekable source into `n` chunks,
/// adjusting boundaries so that each chunk ends at a newline.
pub fn divide_chunks<R: Read + Seek>(reader: &mut R, n: usize) -> std::io::Result<Vec<Chunk>> {
    let file_size = reader.seek(SeekFrom::End(0))?;
    let chunk_size = file_size / n as u64;
    let mut chunks = Vec::with_capacity(n);
    let mut start = 0u64;

    for i in 0..n {
        if i == n - 1 {
            chunks.push(Chunk { start, end: file_size });
            break;
        }

        let approx_end = start + chunk_size;
        reader.seek(SeekFrom::Start(approx_end))?;

        let mut buf = [0u8; 1];
        let mut end = approx_end;
        loop {
            reader.read_exact(&mut buf)?;
            end += 1;
            if buf[0] == b'\n' {
                break;
            }
        }

        chunks.push(Chunk { start, end });
        start = end;
    }

    Ok(chunks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_cursor(content: &'static [u8]) -> Cursor<&'static [u8]> {
        Cursor::new(content)
    }

    #[test]
    fn test_chunks_cover_entire_file() {
        let mut cursor = make_cursor(b"header\nline1\nline2\nline3\nline4\n");
        let file_size = cursor.seek(SeekFrom::End(0)).unwrap();
        cursor.seek(SeekFrom::Start(0)).unwrap();
        let chunks = divide_chunks(&mut cursor, 2).unwrap();

        assert_eq!(chunks[0].start, 0);
        assert_eq!(chunks.last().unwrap().end, file_size);
    }

    #[test]
    fn test_no_gaps_between_chunks() {
        let mut cursor = make_cursor(b"header\nline1\nline2\nline3\nline4\n");
        let chunks = divide_chunks(&mut cursor, 2).unwrap();

        for i in 0..chunks.len() - 1 {
            assert_eq!(chunks[i].end, chunks[i + 1].start);
        }
    }


    #[test]
    fn test_no_empty_chunks() {
        let mut cursor = make_cursor(
            b"header\nline01\nline02\nline03\nline04\nline05\nline06\nline07\nline08\nline09\nline10\nline11\nline12\n"
        );
        let chunks = divide_chunks(&mut cursor, 4).unwrap();

        for chunk in &chunks {
            assert!(chunk.end > chunk.start);
        }
    }

    #[test]
    fn test_correct_number_of_chunks() {
        let mut cursor = make_cursor(
            b"header\nline01\nline02\nline03\nline04\nline05\nline06\nline07\nline08\nline09\nline10\nline11\nline12\n"
        );
        let chunks = divide_chunks(&mut cursor, 4).unwrap();

        assert_eq!(chunks.len(), 4);
    }    

    #[test]
    fn test_chunks_end_on_newline() {
        let content = b"header\nline1\nline2\nline3\nline4\n";
        let mut cursor = make_cursor(content);
        let chunks = divide_chunks(&mut cursor, 2).unwrap();

        for chunk in &chunks[..chunks.len() - 1] {
            assert_eq!(content[chunk.end as usize - 1], b'\n');
        }
    }
}