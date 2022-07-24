use wasm_bindgen::prelude::*;

use js_sys::Uint8Array;

use num_complex::Complex as Cplx;

use crate::utils::{LCG, set_panic_hook};
use crate::argand::ZPlane;


#[wasm_bindgen]
pub struct Julia {
  z: ZPlane<u8>,
  c: Cplx<f64>, // as in z <-> z*z + c
  a: Cplx<f64>, // attraction point that c moves to
  rng: LCG,
  image: Vec<u8>
}

const MIIM_MAX_DEPTH: u8 = 13;
const IIM_ITERS: u32 = 100000;
const INC: u8 = 4;

// speed at which c is pulled to a
const SPEED: f64 = 0.01;


#[wasm_bindgen]
impl Julia {

  #[wasm_bindgen(constructor)]
  pub fn new(cr: f64, ci: f64, scale: f64, width: u32, height: u32) -> Julia {

    set_panic_hook();

    let mut julia = Julia {
      z: ZPlane::<u8>::new(Cplx::new(-scale, -scale), Cplx::new(scale, scale), width, height),
      c: Cplx::new(cr, ci),
      a: Cplx::new(0.0, 0.0),
      rng: LCG::new(19937),
      image: vec![0u8; (width * height * 4) as usize]
    };
    julia.draw();
    julia
  }

  pub fn image_buffer(&self) -> Uint8Array {
    unsafe { Uint8Array::view(&self.image) }
  }

  pub fn cells(&self) -> *const u8 {
    self.z.cells.as_ptr()
  }

  pub fn locus_r(&self) -> u32 {
    ((self.c.re - self.z.zmin.re) * self.z.rscale) as u32
  }

  pub fn locus_i(&self) -> u32 {
    ((self.c.im - self.z.zmin.im) * self.z.iscale) as u32
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
    let mut next = vec![0u8; (self.z.width * self.z.height) as usize];
    let z = self.draw_miim(&mut next);
    self.draw_iim(z, &mut next);
    self.z.cells = next;
  }

  fn draw_iim(&mut self, mut z: Cplx<f64>, next: &mut [u8]) {
    for _ in 0..IIM_ITERS {
      z = (z - self.c).sqrt();
      let mut idx = self.z.get_index(&z);
      if next[idx] > 0 {
        next[idx] += INC;
        z = -z;
        idx = self.z.get_index(&z);
      }
      next[idx] += INC;
    }
  }

  pub fn render(&mut self) {
    for i in 0..((self.z.width * self.z.height) as usize) {
      self.image[i*4] = self.z.cells[i];
      self.image[i*4+1] = self.z.cells[i];
      self.image[i*4+2] = self.z.cells[i];
      self.image[i*4+3] = 255u8;
    }
  }

  // Uses the MIIM algorithm
  fn draw_miim(&mut self, next: &mut Vec::<u8>) -> Cplx<f64> {

    let mut z = Cplx::new(0.0, 0.0);
    let mut sign = 1.0;
    // warmup
    for _ in 0..25 {
      if self.rng.next_1() % 2 == 1 { sign *= -1.0; }
      z = (z - self.c).sqrt() * sign;
    }
    self.draw_miim_impl(z, next, 0);
    z
  }

  fn draw_miim_impl(&mut self, z: Cplx<f64>, cells: &mut Vec<u8>, depth: u8) {

    // copies z - caller will not see changes
    let z = (z - self.c).sqrt();

    let idx = self.z.get_index(&z);
    cells[idx] += INC;
    let idx = self.z.get_index(&-z);
    cells[idx] += INC;
    if depth >= MIIM_MAX_DEPTH { return; }

    self.draw_miim_impl(z, cells, depth+1);
    self.draw_miim_impl(-z, cells, depth+1);
  }

}
