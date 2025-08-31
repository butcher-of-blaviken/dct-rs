// A PGM image represents a grayscale graphic image.
// Each PGM image consists of the following:
// 1. A "magic number" for identifying the file type. A pgm image's magic number is the two characters "P5".
// 2. Whitespace (blanks, TABs, CRs, LFs).
// 3. A width, formatted as ASCII characters in decimal.
// 4. Whitespace.
// 5. A height, again in ASCII decimal.
// 6. Whitespace.
// 7. The maximum gray value (Maxval), again in ASCII decimal. Must be less than 65536, and more than zero.
// 8. A single whitespace character (usually a newline).
// 9. A raster of Height rows, in order from top to bottom. Each row consists of Width gray values, in order
//    from left to right. Each gray value is a number from 0 through Maxval, with 0 being black and Maxval
//    being white. Each gray value is represented in pure binary by either 1 or 2 bytes. If the Maxval is
//    less than 256, it is 1 byte. Otherwise, it is 2 bytes. The most significant byte is first.
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IOError, ErrorKind, Read};

pub struct PGMImage {
    /// The width, formatted as ASCII characters in decimal.
    pub width: usize,
    /// The height, again in ASCII decimal.
    pub height: usize,
    /// The maximum gray value (Maxval), again in ASCII decimal.
    /// Must be less than 65536, and more than zero.
    pub maxval: u16,
    /// The raster, consists of height rows, in order from top to bottom.
    /// Each row consists of Width gray values, in order from left to right.
    /// Each gray value is a number from 0 through Maxval, with 0 being black and Maxval
    /// being white.
    /// NOTE: only supports maxval of 255.
    pub image8: Vec<u8>,
}

impl Display for PGMImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(width:{}, height:{}, maxval:{}, totalpixels:{})",
            self.width,
            self.height,
            self.maxval,
            self.image8.len()
        )
    }
}

impl PGMImage {
    /// Parse the provided PGM file into a PGMFile structure, suitable
    /// for further processing.
    pub fn parse(pgm_file_path: &String) -> Result<Self, IOError> {
        let file = File::open(pgm_file_path)?;
        let mut buf_reader = BufReader::new(file);

        // 1. read the magic number, which should be P5.
        let mut line = String::new();
        buf_reader.read_line(&mut line)?;
        let magic_number = line.trim();
        if magic_number != "P5" {
            return Err(IOError::new(
                ErrorKind::Other,
                format!("Unsupported PGM type: {}", magic_number),
            ));
        }

        // 2. Read the dimensions
        line.clear();
        buf_reader.read_line(&mut line)?;
        let dimensions: Vec<&str> = line.trim().split_whitespace().collect();
        if dimensions.len() != 2 {
            return Err(IOError::new(
                ErrorKind::Other,
                format!(
                    "expected 2 dimensions, got {} ({:?})",
                    dimensions.len(),
                    &dimensions
                ),
            ));
        }
        let width: usize = dimensions[0]
            .parse()
            .map_err(|e| IOError::new(ErrorKind::Other, format!("failed to parse width: {}", e)))?;
        let height: usize = dimensions[1].parse().map_err(|e| {
            IOError::new(ErrorKind::Other, format!("failed to parse height: {}", e))
        })?;

        // 3. Read the maxval
        line.clear();
        buf_reader.read_line(&mut line)?;
        let maxval: u16 = line.trim().parse().map_err(|e| {
            IOError::new(ErrorKind::Other, format!("failed to parse maxval: {}", e))
        })?;

        // 4. Read the pixels
        // NOTE: we don't really do input validation on the pixels here,
        // e.g. whether they're truly < maxval.
        let mut pixels = vec![0; width * height];
        buf_reader.read_exact(&mut pixels)?;

        Ok(PGMImage {
            width: width,
            height: height,
            maxval: maxval,
            image8: pixels,
        })
    }
}

mod tests {
    use crate::pgm_parse::PGMImage;

    #[test]
    fn test_parse_pgm() {
        let path = "./test.pgm".to_string();
        let pgm_image_result = PGMImage::parse(&path);
        assert!(pgm_image_result.is_ok());
        let pgm_image = pgm_image_result.unwrap();
        assert_eq!(pgm_image.height, 512);
        assert_eq!(pgm_image.width, 512);
        assert_eq!(pgm_image.maxval, 255);
        assert_eq!(pgm_image.image8.len(), 512 * 512);
    }
}
