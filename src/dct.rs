use std::f64::consts::PI;

/// dct_block performs the 2D DCT algorithm on the given 8x8 block.
/// Suppose g is the original block.
/// The DCT is G, and given by:
/// G(u, v) = 0.25 * alpha(u) * alpha(v) *
///           sum(x=0..7) sum(y=0..7)
///             g(x, y) * cos((2x + 1) * u * pi / 16) *
///             cos((2y + 1) * v * pi / 16)
/// where:
/// * u is the horizontal spatial frequency
/// * v is the veritcal spatial frequency
/// * alpha(u) and alpha(v) are normalizing scale factors to make the
/// transformation orthonormal with
/// alpha(i) = 1/sqrt(2) if i == 0 else 1
/// * g(x, y) is the pixel value at coordinates (x, y)
/// * G(u, v) is the DCT coefficient at coordinates (u, v)
pub fn dct_block(width: usize, height: usize, block: &Vec<u8>) -> Vec<f64> {
    const BLOCK_SIZE: usize = 8;
    debug_assert!(width == BLOCK_SIZE && height == BLOCK_SIZE);

    let shifted = level_shift_block(&block);
    let mut dct = vec![0.0; shifted.len()];

    for i in 0 as usize..width {
        for j in 0 as usize..height {
            let preamble = 0.25 * alpha(i) * alpha(j);
            let mut sum = 0.0;
            for x in 0 as usize..width {
                for y in 0 as usize..height {
                    let cos_x = (((2 * x + 1) as f64) * (i as f64) * PI / 16.0).cos();
                    let cos_y = (((2 * y + 1) as f64) * (j as f64) * PI / 16.0).cos();
                    sum += shifted[BLOCK_SIZE * x + y] * cos_x * cos_y;
                }
            }
            dct[BLOCK_SIZE * i + j] = preamble * sum;
        }
    }

    dct
}

/// alpha is the normalizing scale factor to make the DCT transformation
/// orthonormal.
fn alpha(i: usize) -> f64 {
    if i == 0 {
        return 1.0 / (2 as f64).sqrt();
    }
    1.0
}

/// level_shift_block performs a level shift of each element
/// by subtracting it by 128.0f64.
fn level_shift_block(block: &Vec<u8>) -> Vec<f64> {
    let mut result = vec![0.0; block.len()];
    for (i, val) in block.iter().enumerate() {
        result[i] = (*val as f64) - 128.0;
    }
    result
}

mod tests {
    use crate::{
        dct::{dct_block, level_shift_block},
        pgm_parse::PGMImage,
    };

    #[test]
    fn test_level_shift_block() {
        let path = "./8by8_grayscale.pgm".to_string();
        let pgm_image_result = PGMImage::parse(&path);
        assert!(pgm_image_result.is_ok());
        let pgm_image = pgm_image_result.unwrap();
        assert_eq!(pgm_image.height, 8);
        assert_eq!(pgm_image.width, 8);
        let shifted_block = level_shift_block(&pgm_image.image8);
        for (i, _) in shifted_block.iter().enumerate() {
            assert_eq!(shifted_block[i], (pgm_image.image8[i] as f64) - 128.0)
        }
    }

    #[test]
    fn test_dct_block() {
        let path = "./8by8_grayscale.pgm".to_string();
        let pgm_image_result = PGMImage::parse(&path);
        assert!(pgm_image_result.is_ok());
        let pgm_image = pgm_image_result.unwrap();
        assert_eq!(pgm_image.height, 8);
        assert_eq!(pgm_image.width, 8);
        let dct = dct_block(pgm_image.width, pgm_image.height, &pgm_image.image8);
        println!("first 8 elements: {:?}", &dct[0..8])
    }
}
