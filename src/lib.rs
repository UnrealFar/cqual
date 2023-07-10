#![warn(missing_debug_implementations)]

mod uniform_quantization;
use std::path::Path;

pub use image::ImageResult;
pub use uniform_quantization::Quantizer as UniformQuantizer;

pub trait Quantize {
    fn quantize(&mut self, pixels: &[(u8, u8, u8, u8)]);
    fn reset(&mut self) {}
    fn get_quantized(&self, r: u8, g: u8, b: u8, a: u8) -> (u8, u8, u8, u8);
    fn get_pallet(&self);
}

pub trait QuantizeImage {
    fn quantize_image<P: AsRef<Path>>(&mut self, input: P, output: P) -> ImageResult<()>;
}

impl<T> QuantizeImage for T
where
    T: Quantize,
{
    fn quantize_image<P: AsRef<Path>>(&mut self, input: P, output: P) -> ImageResult<()> {
        let mut img = image::open(input)?.into_rgba8();

        let pixels = img
            .pixels()
            .into_iter()
            .map(|x| (x.0[0], x.0[1], x.0[2], x.0[3]))
            .collect::<Vec<_>>();

        self.reset();
        self.quantize(pixels.as_slice());

        let (h, w) = img.dimensions();
        for x in 0..h {
            for y in 0..w {
                let pixel = img.get_pixel(x, y);
                let (r, g, b, a) =
                    self.get_quantized(pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3]);
                img.put_pixel(x, y, image::Rgba([r, g, b, a]));
            }
        }

        img.save(output)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_uniform_quantization() {
        let mut quantizer: UniformQuantizer = UniformQuantizer::new(3, 3, 2);

        quantizer
            .quantize_image("assets/img.jpg", "assets/uniform_quantized.jpg")
            .unwrap();
    }
}
