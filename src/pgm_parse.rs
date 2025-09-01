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
    pub image_u8: Vec<u8>,
}

impl Display for PGMImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(width:{}, height:{}, maxval:{}, totalpixels:{})",
            self.width,
            self.height,
            self.maxval,
            self.image_u8.len()
        )
    }
}

impl PGMImage {
    /// num_blocks returns the number of 8x8 blocks in this image.
    pub fn num_blocks(&self, block_size: usize) -> usize {
        self.width * self.height / (block_size * block_size)
    }

    /// get_block returns the 8x8 block at the given block index.
    /// For example, a 512x512 image will have 4,096 8x8 blocks in total.
    pub fn get_block(&self, block_size: usize, block_index: usize) -> Result<Vec<u8>, String> {
        let num_pixels: usize = block_size * block_size;
        let num_blocks = (self.width * self.height) / (block_size * block_size);
        if block_index >= num_blocks {
            return Err(format!(
                "out of bounds block index {}, max is {}",
                block_index, num_blocks
            ));
        }

        // translate the block index into a starting index into image8
        let start_index = self.translate_index(block_size, block_index);
        let mut ret = vec![0 as u8; num_pixels];
        let mut ret_idx: usize = 0;
        // navigate the array to create the 8x8 block in column-major order.
        // TODO: there a way to do this in row-major order instead for consistency?
        for i in start_index..start_index + 8 {
            for j in 0..8 {
                ret[ret_idx] = self.image_u8[i + self.width * j];
                ret_idx += block_size;
                if ret_idx >= num_pixels {
                    ret_idx = (ret_idx % num_pixels) + 1;
                }
            }
        }

        Ok(ret)
    }

    fn translate_index(&self, block_size: usize, block_index: usize) -> usize {
        let col_idx = block_index % (self.width / block_size);
        let row_idx = block_index / (self.height / block_size);
        let row_addend = if row_idx >= 1 {
            (row_idx + 1) * self.width
        } else {
            0
        };
        let col_addend = col_idx * block_size;
        row_addend + col_addend
    }

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
            image_u8: pixels,
        })
    }
}

mod tests {
    use crate::{consts::BLOCK_SIZE_8X8, pgm_parse::PGMImage};

    #[test]
    fn test_parse_pgm() {
        let path = "./testdata/test.pgm".to_string();
        let pgm_image_result = PGMImage::parse(&path);
        assert!(pgm_image_result.is_ok());
        let pgm_image = pgm_image_result.unwrap();
        assert_eq!(pgm_image.height, 512);
        assert_eq!(pgm_image.width, 512);
        assert_eq!(pgm_image.maxval, 255);
        assert_eq!(pgm_image.image_u8.len(), 512 * 512);
    }

    #[test]
    fn test_get_block() {
        let path = "./testdata/8by8_grayscale.pgm".to_string();
        let pgm_image_result = PGMImage::parse(&path);
        assert!(pgm_image_result.is_ok());
        let pgm_image = pgm_image_result.unwrap();
        let first_block_result = pgm_image.get_block(BLOCK_SIZE_8X8, 0);
        assert!(first_block_result.is_ok());
        let first_block = first_block_result.unwrap();
        for (i, _) in pgm_image.image_u8.iter().enumerate() {
            assert_eq!(
                first_block[i], pgm_image.image_u8[i],
                "testing at index {}",
                i
            );
        }
    }

    #[test]
    fn test_get_block_second_block_index() {
        let path = "./testdata/16by16_grayscale.pgm".to_string();
        let pgm_image_result = PGMImage::parse(&path);
        assert!(pgm_image_result.is_ok());
        let pgm_image = pgm_image_result.unwrap();
        let second_block_result = pgm_image.get_block(BLOCK_SIZE_8X8, 1);
        assert!(second_block_result.is_ok());
        let second_block = second_block_result.unwrap();
        assert_eq!(&second_block[0..8], &pgm_image.image_u8[8..16]);
        assert_eq!(&second_block[8..16], &pgm_image.image_u8[24..32]);
        assert_eq!(&second_block[16..24], &pgm_image.image_u8[40..48]);
        assert_eq!(&second_block[24..32], &pgm_image.image_u8[56..64]);
    }

    #[test]
    fn test_get_block_third_block_index() {
        let path = "./testdata/16by16_grayscale.pgm".to_string();
        let pgm_image_result = PGMImage::parse(&path);
        assert!(pgm_image_result.is_ok());
        let pgm_image = pgm_image_result.unwrap();
        let third_block_result = pgm_image.get_block(BLOCK_SIZE_8X8, 2);
        assert!(third_block_result.is_ok());
        let third_block = third_block_result.unwrap();
        assert_eq!(&third_block[0..8], &pgm_image.image_u8[32..40]);
        // assert_eq!(&third_block[8..16], &pgm_image.image_u8[24..32]);
        // assert_eq!(&third_block[16..24], &pgm_image.image_u8[40..48]);
        // assert_eq!(&third_block[24..32], &pgm_image.image_u8[56..64]);
    }

    #[test]
    fn test_num_blocks() {
        let path = "./testdata/16by16_grayscale.pgm".to_string();
        let pgm_image_result = PGMImage::parse(&path);
        assert!(pgm_image_result.is_ok());
        let pgm_image = pgm_image_result.unwrap();
        assert_eq!(pgm_image.num_blocks(BLOCK_SIZE_8X8), (16 * 16) / (8 * 8));
    }

    #[test]
    fn test_translate_index() {
        let path = "./testdata/16by16_grayscale.pgm".to_string();
        let pgm_image_result = PGMImage::parse(&path);
        assert!(pgm_image_result.is_ok());
        let pgm_image = pgm_image_result.unwrap();

        assert_eq!(pgm_image.translate_index(BLOCK_SIZE_8X8, 0), 0);
        assert_eq!(pgm_image.translate_index(BLOCK_SIZE_8X8, 1), 8);
        assert_eq!(pgm_image.translate_index(BLOCK_SIZE_8X8, 2), 32);
        assert_eq!(pgm_image.translate_index(BLOCK_SIZE_8X8, 3), 40);
    }

    #[test]
    fn test_translate_index_64x64() {
        let path = "./testdata/64x64_grayscale.pgm".to_string();
        let pgm_image_result = PGMImage::parse(&path);
        assert!(pgm_image_result.is_ok());
        let pgm_image = pgm_image_result.unwrap();

        assert_eq!(pgm_image.translate_index(BLOCK_SIZE_8X8, 0), 0);
        assert_eq!(pgm_image.translate_index(BLOCK_SIZE_8X8, 1), 8);
        assert_eq!(pgm_image.translate_index(BLOCK_SIZE_8X8, 2), 16);
        assert_eq!(pgm_image.translate_index(BLOCK_SIZE_8X8, 3), 24);

        assert_eq!(pgm_image.translate_index(BLOCK_SIZE_8X8, 8), 128);
        assert_eq!(pgm_image.translate_index(BLOCK_SIZE_8X8, 16), 192);
    }
}
