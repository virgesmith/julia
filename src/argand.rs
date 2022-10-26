
use num_complex::Complex as Cplx;
use num_traits::{Unsigned, Zero};


pub struct ZPlane<T> {
  pub zmin: Cplx<f64>, // bottom left
  pub zmax: Cplx<f64>, // top right
  pub rscale: f64,
  pub iscale: f64,
  pub width: u32,
  pub height: u32,
  pub cells: Vec<T>,
}

impl<T: Zero + Unsigned + Clone> ZPlane<T> {

  pub fn new(zmin: Cplx<f64>, zmax: Cplx<f64>, width: u32, height: u32) -> ZPlane<T> {

    ZPlane {
      zmin,
      zmax,
      width,
      height,
      rscale: width as f64 / (zmax.re - zmin.re),
      iscale: height as f64 / (zmax.im - zmin.im),
      cells: vec![T::zero(); (width * height) as usize]
    }
  }

  pub fn index_from_point(&self, z: &Cplx<f64>) -> usize {
    let r = ((z.re - self.zmin.re) * self.rscale) as u32;
    let c = ((z.im - self.zmin.im) * self.iscale) as u32;
    (c * self.width + r) as usize
  }

  pub fn point_from_rc(&self, r: u32, c: u32) -> (Cplx::<f64>, usize) {
    (Cplx::new(r as f64 / self.rscale + self.zmin.re,
              c as f64 / self.iscale + self.zmin.im),
    (c * self.width + r) as usize)
  }

  pub fn point_from_index(&self, i: usize) -> Cplx::<f64> {
    let (c, r) = (i as u32 / self.height, i as u32 % self.width);
    Cplx::new(r as f64 / self.rscale + self.zmin.re,
              c as f64 / self.iscale + self.zmin.im)
  }

}



