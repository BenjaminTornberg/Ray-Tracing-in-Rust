

use crate::{vector::{Color, Point3, Vec3}, perlin::Perlin};


use image::{io::Reader as ImageReader, GenericImageView, Rgba};

pub trait Tex{
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

#[derive(Debug, Clone)]
pub enum Texture{
    SolidColor(SolidColor),
    CheckeredTexture(CheckeredTexture),
    NoiseTexture(NoiseTexture),
    ImageTexture(ImageTexture)
}
impl Tex for Texture{
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        match self{
            Texture::SolidColor(solidcolor) => solidcolor.value(u, v, p),
            Texture::CheckeredTexture(checkered) => checkered.value(u, v, p),
            Texture::NoiseTexture(noise) => noise.value(u, v, p),
            Texture::ImageTexture(image) => image.value(u, v, p),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SolidColor{
    pub color_value: Color
}
impl SolidColor{
    pub fn new(red: f64, green: f64, blue: f64)-> SolidColor { SolidColor { color_value: Vec3::color(red, green, blue) }}
}
impl Tex for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color_value
    }
}

#[derive(Debug, Clone)]
pub struct CheckeredTexture{
    pub even: Box<Texture>,
    pub odd: Box<Texture>
}
impl CheckeredTexture{
    pub fn new(even: Texture, odd: Texture) -> CheckeredTexture { CheckeredTexture { even: Box::new(even), odd: Box::new(odd) }}
    pub fn new_rgb(even: Color, odd: Color) -> CheckeredTexture {
        CheckeredTexture{ 
            even: Box::new(Texture::SolidColor(SolidColor{color_value: even})),
            odd: Box::new(Texture::SolidColor(SolidColor{color_value: odd}))
        }
    }
}

impl Tex for CheckeredTexture{
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0*p.x()).sin() * (10.0*p.y()).sin() * (10.0*p.z()).sin();
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        }else {
            return self.even.value(u, v, p);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NoiseTexture{
    noise: Perlin,
    scale: f64,
}
impl NoiseTexture{
    pub fn new(scale: f64) -> NoiseTexture{ NoiseTexture { noise: Perlin::new(), scale}}
}
impl Tex for NoiseTexture{
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Vec3::color(1.0, 1.0, 1.0) * 0.5 * ( 1.0 + (self.scale*p.z() + 10.0*self.noise.turb(p, None)).sin())
    }
}

#[derive(Debug, Clone)]
pub struct ImageTexture{
    data: Vec<Rgba<u8>>,
    width: u32,
    height: u32,
}
impl ImageTexture{
    pub fn new(path: &str) -> ImageTexture {
        let img = ImageReader::open(path).expect(path).decode().unwrap();

        let pixels: Vec<_> = img.pixels().map(|(_, _, rgb)| rgb).collect();

        let (width, height) = img.dimensions();
        ImageTexture { data: pixels, width, height }
    }
}
impl Tex for ImageTexture{
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        let uu = u * self.width as f64;
        let vv = (1.0 - v) * (self.height - 1) as f64;

        let color_scale = 1.0 / 255.0;

        let pix_ind = ((vv.floor() as u64) * self.width as u64 + (uu.floor() as u64)) as usize;
        let pixel = self.data[pix_ind];
        
        //let pixel = self.data.get_pixel(i, j).0;
        Vec3::color(
            pixel[0] as f64 * color_scale,
            pixel[1] as f64 * color_scale,
            pixel[2] as f64 * color_scale,
        )
    }   
}