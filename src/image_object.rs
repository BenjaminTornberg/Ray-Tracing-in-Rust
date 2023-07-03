use image::{Rgb, RgbImage};
use crate::utils::clamp;
use crate::vector::Color;


pub struct ImageParams{
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub background: Color,
}
impl ImageParams{
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32, max_depth: u32, background: Color) -> ImageParams{
        ImageParams{
            aspect_ratio,
            image_width,
            image_height: (image_width as f64 / aspect_ratio) as u32,
            samples_per_pixel,
            max_depth,
            background
        }
    }
}
pub struct Image{
    width: u32,
    height: u32,
    pixels: RgbImage,
}

impl Image{
    pub fn new(width: u32, height: u32) -> Image{
        Image{
            width,
            height,
            pixels: RgbImage::new(width, height),

        }
    }

    pub fn set_pixel(&mut self, x:u32, y:u32, pixel_color: Color, samples_per_pixel: u32){
        //divide the color by the number of samples and gamma-correct for gamma=2
        let scale = 1.0 / samples_per_pixel as f64;
        let r = (pixel_color.r() * scale).sqrt();
        let g = (pixel_color.g() * scale).sqrt();
        let b = (pixel_color.b() * scale).sqrt();

        let r = (255.0 * clamp(r, 0.0, 0.999)) as u8;
        let g = (255.0 * clamp(g, 0.0, 0.999)) as u8;
        let b = (255.0 * clamp(b, 0.0, 0.999)) as u8;


        self.pixels.put_pixel(x, y, Rgb([r, g, b]));
    }

    pub fn output(&self){
        println!("P3");
        println!("{} {}", self.width, self.height);
        println!("255");

        for y in (0..self.height).rev(){
            for x in 0..self.width{
                let pixel = self.pixels.get_pixel(x, y);
                let rgb = pixel.0;
                //fucking makes zero sense why the blue and red got switched
                let r = rgb[0];
                let g = rgb[1];
                let b = rgb[2];
                println!("{r} {g} {b}");
            }
        }   
    }
}