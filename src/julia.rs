use wasm_bindgen::prelude::*;

use js_sys::Uint8Array;

//use rayon::prelude::*;
use num_complex::Complex as Cplx;

use crate::utils::{set_panic_hook, colour_map};
use crate::argand::ZPlane;

type Cell = u8;

#[wasm_bindgen]
pub struct Julia {
  z: ZPlane<Cell>,
  c: Cplx<f64>, // as in z <-> z*z + c
  a: Cplx<f64>, // attraction point that c moves to
  image: Vec<u8>,
  colour_map: Vec<[u8; 4]>
}

// speed at which c is pulled to a
const SPEED: f64 = 0.01;

const MAXITER: Cell = 254;
const ITER_INC: Cell = 2;


#[wasm_bindgen]
impl Julia {

  #[wasm_bindgen(constructor)]
  pub fn new(cr: f64, ci: f64, scale: f64, width: u32, height: u32) -> Julia {

    set_panic_hook();

    let mut julia = Julia {
      z: ZPlane::<Cell>::new(Cplx::new(-scale, -scale), Cplx::new(scale, scale), width, height),
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

  fn iterate(&self, i: usize) -> Cell{
    let (x, y) = (i as u32 / self.z.height, i as u32 % self.z.width);
      let (mut z, _) = self.z.get_point(y, x);
      let mut iter: Cell = 0;
      while z.norm_sqr() < 400. && iter < MAXITER {
        z = z * z + self.c;
        iter += ITER_INC;
      }
    iter
  }

  fn draw(&mut self) {
    // iterate over half the plane (symmetry)
    let n = (self.z.width * self.z.height / 2) as usize;
    let mut next = vec![0 as Cell; n];

    next.iter_mut().enumerate().for_each(|(i, n)| *n = self.iterate(i));
    self.z.cells[0..n].copy_from_slice(&next);
    next.reverse();
    self.z.cells[n..].copy_from_slice(&next);
  }

  pub fn render(&mut self) {

    self.image = (0..(self.z.width * self.z.height) as usize)
      .flat_map(|i| self.colour_map[self.z.cells[i] as usize].clone())
      .collect::<Vec<_>>();

    // plot the locus
    let idx = self.z.get_index(&self.c);
    self.image.splice(idx*4..(idx+1)*4, [0, 0, 0, 255]);
  }

  pub fn image_buffer(&self) -> Uint8Array {
    unsafe { Uint8Array::view(&self.image) }
  }
}
