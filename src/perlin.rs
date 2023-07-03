use crate::{utils::random_int_range, vector::{Point3, unit_vector, dot}};
use crate::vector::Vec3;

const POINT_COUNT: usize = 256;

#[derive(Debug, Clone, Copy)]
pub struct Perlin{
    ranvec: [Vec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT]
}
impl Perlin{
    pub fn new() -> Perlin{
        let mut ranvec = [Vec3(0.0, 0.0, 0.0); POINT_COUNT];

        for i in 0..POINT_COUNT{
            ranvec[i] = unit_vector(Vec3::random_range(-1.0, 1.0));
        }

        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();
        Perlin { ranvec, perm_x, perm_y, perm_z}
    }

    fn perlin_generate_perm() -> [i32; POINT_COUNT]{
        //I kinda hate this line of code
        let mut p: [i32; POINT_COUNT] = (0..POINT_COUNT as i32).collect::<Vec<_>>().try_into().unwrap();

        for i in (1..POINT_COUNT).rev(){
            let target = random_int_range(0, i as i32) as usize;
            (p[i], p[target]) = (p[target], p[i])
        }

        p
    }

    pub fn noise(&self, p: &Point3) -> f64{
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[Vec3(0.0, 0.0, 0.0);2];2];2];

        for di in 0..2{
            for dj in 0..2{
                for dk in 0..2{
                    c[di][dj][dk] = self.ranvec[(
                        self.perm_x[(i as usize + di) & 255] ^ 
                        self.perm_y[(j as usize + dj) & 255] ^ 
                        self.perm_z[(k as usize + dk) & 255]) as usize];
                }
            }
        }
        Perlin::trilinear_interp(c, u, v, w)
    }
    fn trilinear_interp(c: [[[Vec3;2];2];2], u: f64, v: f64, w: f64) -> f64{
        let uu = u*u*(3.0-2.0*u);
        let vv = v*v*(3.0-2.0*v);
        let ww = w*w*(3.0-2.0*w);
        let mut accum = 0.0;
        for i in 0..2{
            for j in 0..2{
                for k in 0..2{
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;
                    let weight_v = Vec3(u - i, v - j, w - k);
                    accum += (i*uu + (1.0-i)*(1.0-uu))*
                    (j*vv + (1.0-j)*(1.0-vv))*
                    (k*ww + (1.0-k)*(1.0-ww))*dot(c[i as usize][j as usize][k as usize], weight_v);
                }
            }
        }
        accum
    }
    pub fn turb(&self, p: &Point3, depth: Option<i32>) -> f64 {
        let mut accum = 0.0;
        let mut tmp_p = *p;
        let mut weight = 1.0;
        let depth = depth.unwrap_or(7);

        for _ in 0..depth{
            accum += weight * self.noise(&tmp_p);
            weight *= 0.5;
            tmp_p *= 2.0;
        }
        accum.abs()
    }

        
}

