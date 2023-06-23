
const WIDTH: i32 = 256;
const HEIGHT: i32 = 256;

fn main(){
    println!("P3");
    println!("{} {}", WIDTH, HEIGHT);
    println!("255");
    for j in (0..256).rev(){
        for i in 0..256{
            let r = i as f64 / 255 as f64;
            let g = j as f64 / 255 as f64;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ib = (255.999 * b) as i32;
            let ig = (255.999 * g) as i32;

            println!("{} {} {}", ir, ib, ig);
        }
    }

}