use wasm_bindgen::prelude::*;

use js_sys::Uint8Array;

use num_complex::Complex as Cplx;

use crate::utils::{set_panic_hook, colour_map};
use crate::argand::ZPlane;


#[wasm_bindgen]
pub struct Julia {
  z: ZPlane<u8>,
  c: Cplx<f64>, // as in z <-> z*z + c
  a: Cplx<f64>, // attraction point that c moves to
  image: Vec<u8>,
  colour_map: Vec<Vec<u8>>
}

// speed at which c is pulled to a
const SPEED: f64 = 0.01;

const MAXITER: u8 = 255;


#[wasm_bindgen]
impl Julia {

  #[wasm_bindgen(constructor)]
  pub fn new(cr: f64, ci: f64, scale: f64, width: u32, height: u32) -> Julia {

    set_panic_hook();

    let mut julia = Julia {
      z: ZPlane::<u8>::new(Cplx::new(-scale, -scale), Cplx::new(scale, scale), width, height),
      c: Cplx::new(cr, ci),
      a: Cplx::new(0.0, 0.0),
      image: vec![0u8; (width * height * 4) as usize],
      colour_map: colour_map(512)
    };
    julia.draw();
    julia
  }

  pub fn set_attract(&mut self, row: u32, col: u32) {
    let (c, _) = self.z.get_point(row, col);
    self.a = c;
  }

  pub fn tick(&mut self) {
    self.c += Cplx::new((self.a.re - self.c.re) * SPEED, (self.a.im - self.c.im) * SPEED);
    if self.c.re > self.z.zmax.re { self.c.re = self.z.zmax.re; }
    if self.c.re < self.z.zmin.re { self.c.re = self.z.zmin.re; }
    if self.c.im > self.z.zmax.im { self.c.im = self.z.zmax.im; }
    if self.c.im < self.z.zmin.im { self.c.im = self.z.zmin.im; }

    self.draw();
  }

  fn draw(&mut self) {
    let n = (self.z.width * self.z.height) as usize;
    let mut next = vec![0u8; n];

    for y in 0..self.z.height / 2 {
      for x in 0..self.z.width {
        let (mut z, _) = self.z.get_point(y, x);
        let mut iter = 0u8;
        while z.norm_sqr() < 400. && iter < MAXITER {
          z = z * z + self.c;
          iter += 5;
        }
        let p = (y + self.z.height * x) as usize;
        next[p] = iter;
        next[n-p-1] = iter;
      }
    }
    self.z.cells = next;
  }

  pub fn render(&mut self) {
    for i in 0..((self.z.width * self.z.height) as usize) {
      self.image[i*4] = self.colour_map[self.z.cells[i] as usize][0];
      self.image[i*4+1] = self.colour_map[self.z.cells[i] as usize][1];
      self.image[i*4+2] = self.colour_map[self.z.cells[i] as usize][2];
      self.image[i*4+3] = 255u8;
    }

    // plot the locus
    let idx = self.z.get_index(&self.c);
    self.image[idx*4] = 0u8;
    self.image[idx*4+1] = 0u8;
    self.image[idx*4+2] = 0u8;
    self.image[idx*4+3] = 255u8;
  }

  pub fn image_buffer(&self) -> Uint8Array {
    unsafe { Uint8Array::view(&self.image) }
  }
}
