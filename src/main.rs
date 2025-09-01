use clap::{Parser, Subcommand};

mod consts;
mod dct;
mod pgm_parse;

/// A user has an uncompressed grayscale image (e.g., 512x512 pixels, each pixel value 0-255)
/// and needs to compress it to reduce file size while maintaining reasonable visual quality.
///
/// They want to understand the core concept behind JPEG compression by implementing the fundamental
/// DCT-based compression on 8x8 pixel blocks.
///
/// Inputs:
/// * Uncompressed grayscale image as a 2D array of integers (0-255)
/// * Image dimensions that are multiples of 8 (for simplicity)
/// * Quality parameter (controls how much compression vs quality loss)
/// Outputs:
/// * Compressed representation of the image (quantized DCT coefficients)
/// * Decompressed/reconstructed image showing the lossy compression effects
/// * Compression ratio achieved
#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compresses a grayscale image using DCT (Discrete Cosine Transform).
    Compress {
        /// Path to the image file to compress. It must be a PGM file in binary format (header P5).
        path: String,
        /// Quality of the compression, range from 0 to 100, with 100 being highest quality
        /// and 0 being lowest.
        quality: u8,
    },
    /// Decompresses a DCT compressed image.
    Decompress {
        /// Path to the image file to decompress. It must be a PGM file.
        path: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match &args.command {
        Commands::Compress { path, quality } => {
            println!("compressing image {} with quality {}...", path, quality);
            let img = pgm_parse::PGMImage::parse(path)?;
            println!("parsed image at path {}, details: {}", path, img);
            Ok(())
        }
        Commands::Decompress { path } => {
            println!("decompressing image {}...", path);
            Ok(())
        }
    }
}
