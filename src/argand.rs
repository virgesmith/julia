
use num_complex::Complex as Cplx;
use num_traits::{Unsigned, Zero};

use crate::utils::ImageSize;


pub struct ZPlane<T> {
  pub zmin: Cplx<f64>, // bottom left
  pub zmax: Cplx<f64>, // top right
  pub scale: Cplx<f64>,
  pub size: ImageSize,
  pub cells: Vec<T>,
}

impl<T: Zero + Unsigned + Clone> ZPlane<T> {
  pub fn new(zmin: Cplx<f64>, zmax: Cplx<f64>, size: ImageSize) -> ZPlane<T> {
    ZPlane {
      zmin,
      zmax,
      scale: Cplx::new(size.0 as f64 / (zmax.re - zmin.re), size.1 as f64 / (zmax.im - zmin.im)),
      size,
      cells: vec![T::zero(); (size.0 * size.1) as usize]
    }
  }

  pub fn index_from_rc(&self, rc: (u32, u32)) -> usize {
    (rc.0 * self.size.0 + rc.1) as usize
  }

  pub fn rc_from_index(&self, idx: usize) -> (u32, u32) {
    (idx as u32 / self.size.0, idx as u32 % self.size.0)
  }

  pub fn point_from_rc(&self, rc: (u32, u32)) -> Cplx::<f64> {
    Cplx::new(rc.1 as f64 / self.scale.re + self.zmin.re,
              rc.0 as f64 / self.scale.im + self.zmin.im)
  }

  pub fn rc_from_point(&self, z: &Cplx<f64>) -> (u32, u32) {
    let c = ((z.re - self.zmin.re) * self.scale.re) as u32;
    let r = ((z.im - self.zmin.im) * self.scale.im) as u32;
    (r, c)
  }

  pub fn index_from_point(&self, z: &Cplx<f64>) -> usize {
    self.index_from_rc(self.rc_from_point(z))
  }

  pub fn point_from_index(&self, idx: usize) -> Cplx::<f64> {
    self.point_from_rc(self.rc_from_index(idx))
  }

}



