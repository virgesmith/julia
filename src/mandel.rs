use wasm_bindgen::prelude::*;

//use complex::Cplx;
use num_complex::Complex as Cplx;
use js_sys::Uint8Array;

use crate::utils::{colour_map, set_panic_hook};
use crate::argand::ZPlane;

type Cell = u16;

#[wasm_bindgen]
pub struct Mandel {
  z: ZPlane<Cell>,
  depth: Cell,
  image: Vec<u8>
}

#[wasm_bindgen]
impl Mandel {

  #[wasm_bindgen(constructor)]
  pub fn new(width: u32, height: u32, maxiter: Cell) -> Mandel {

    set_panic_hook();

    let bottom_left = Cplx::<f64>::new(-2.0, -1.25);
    let top_right = Cplx::<f64>::new(0.5, 1.25);

    let mut mandel = Mandel {
      z: ZPlane::<Cell>::new(bottom_left, top_right, width, height),
      depth: maxiter - 1,
      image: vec![0u8; (width * height * 4) as usize]
    };

    mandel.draw();
    mandel
  }

  pub fn mid_r(&self) -> f64 {
    (self.z.zmin.re + self.z.zmax.re) / 2.0
  }

  pub fn mid_i(&self) -> f64 {
    (self.z.zmin.im + self.z.zmax.im) / 2.0
  }

  pub fn scale(&self) -> f64 {
    self.z.zmax.re - self.z.zmin.re
  }

  // factor > 1 zooms in
  pub fn zoom(&mut self, row: u32, col: u32, factor: f64) {
    let (c, _) = self.z.get_point(row, col);
    self.z.rscale *= factor;
    self.z.iscale *= factor;
    let dr = (self.z.zmax.re - self.z.zmin.re) / (2.0 * factor);
    let di = (self.z.zmax.im - self.z.zmin.im) / (2.0 * factor);
    self.z.zmin.re = c.re - dr;
    self.z.zmin.im = c.im - di;
    self.z.zmax.re = c.re + dr;
    self.z.zmax.im = c.im + di;
    self.draw();
  }

  fn draw(&mut self) {

    for row in 0..self.z.height {
      for col in 0..self.z.width {
        let (c, idx) = self.z.get_point(row, col);
        let mut z = Cplx::new(0.0, 0.0);
        let mut it: Cell = 0;
        let mut r2 = 0.0;
        let mut i2 = 0.0;
        let mut z_prev = z;
        let mut period = 0;
        while it < self.depth && (r2 + i2) < 4.0 {
          //z = z * z + c;
          // hand optimised
          z.im = (z.re + z.re) * z.im + c.im;
          z.re = r2 - i2 + c.re;
          r2 = z.re * z.re;
          i2 = z.im * z.im;
          it += 1;

          // check for periodicity and break early
          if z == z_prev {
            it = self.depth;
            break;
          }
          period += 1;
          // update ref value every 20th iteration (can detect cycles up to 20 iterations long)
          if period > 20 {
            period = 0;
            z_prev = z;
          }
        }
        self.z.cells[idx] = it;
      }
    }
  }

  pub fn render(&mut self) {

    let cmap = colour_map(self.depth as usize);

    for i in 0..((self.z.width * self.z.height) as usize) {
      self.image[i*4] = cmap[self.z.cells[i] as usize][0];
      self.image[i*4+1] = cmap[self.z.cells[i] as usize][1];
      self.image[i*4+2] = cmap[self.z.cells[i] as usize][2];
      self.image[i*4+3] = 255u8;
    }
  }

  pub fn image_buffer(&self) -> Uint8Array {
    unsafe { Uint8Array::view(&self.image) }
  }
}
