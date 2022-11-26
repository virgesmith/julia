use wasm_bindgen::prelude::*;

use js_sys::Uint8Array;

//use rayon::prelude::*;
use num_complex::Complex as Cplx;
use std::cmp::{max, min};

use crate::mandel::Mandel;
use crate::utils::{set_panic_hook, colour_map};
use crate::argand::ZPlane;

type Cell = u8;

#[wasm_bindgen]
pub struct Julia {
  z: ZPlane<Cell>,
  c: Cplx<f64>, // as in z <-> z*z + c
  a: Cplx<f64>, // attraction point that c moves to
  image: Vec<u8>,
  colour_map: Vec<[u8; 4]>,
  overlay_image: Vec<u8>,
}

// speed at which c is pulled to a
const SPEED: f64 = 0.01;

const MAXITER: Cell = 254;
const ITER_INC: Cell = 1;


#[wasm_bindgen]
impl Julia {

  #[wasm_bindgen(constructor)]
  pub fn new(cr: f64, ci: f64, scale: f64, width: u32, height: u32) -> Julia {
    set_panic_hook();

    let xscale = scale;
    let yscale = scale * height as f64 / width as f64;
    let bottom_left = Cplx::new(-xscale, -yscale);
    let top_right = Cplx::new(xscale, yscale);

    let mut mandel = Mandel::custom(bottom_left, top_right, width, height, 512, 512, (2,2,2), 255);
    mandel.render();

    let mut julia = Julia {
      z: ZPlane::<Cell>::new(bottom_left, top_right, width, height),
      c: Cplx::new(cr, ci),
      a: Cplx::new(0.0, 0.0),
      image: vec![0u8; (width * height * 4) as usize],
      colour_map: colour_map(512, (3, 3, 1), 192),
      overlay_image: mandel.raw_image()
    };
    julia.draw();
    julia
  }

  pub fn set_attract(&mut self, row: u32, col: u32) {
    self.a = self.z.point_from_rc((row, col));
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
    let mut z = self.z.point_from_index(i);
    let mut iter: Cell = 0;
    let mut r2 = z.re * z.re;
    let mut i2 = z.im * z.im;
    while r2 + i2 < self.z.scale.re /*400.*/ && iter < MAXITER {
      // z = z * z + self.c;
      z.im = (z.re + z.re) * z.im + self.c.im;
      z.re = r2 - i2 + self.c.re;
      r2 = z.re * z.re;
      i2 = z.im * z.im;
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
      .flat_map(|i| self.colour_map[self.z.cells[i] as usize])
      .collect::<Vec<_>>();

    // plot the locus
    let (x, y) = self.z.rc_from_point(&self.c);
    let idx = self.z.index_from_rc((x, y));
    self.image.splice(idx*4..(idx+1)*4, [0, 0, 0, 255]);

    let x1 = min(x+1, self.z.width-1);
    let idx = self.z.index_from_rc((x1, y));
    self.image.splice(idx*4..(idx+1)*4, [0, 0, 0, 255]);
    let x1 = max(x-1, 0);
    let idx = self.z.index_from_rc((x1, y));
    self.image.splice(idx*4..(idx+1)*4, [0, 0, 0, 255]);

    let y1 = max(y-1, 0);
    let idx = self.z.index_from_rc((x, y1));
    self.image.splice(idx*4..(idx+1)*4, [0, 0, 0, 255]);

    let y1 = min(y+1, self.z.height-1);
    let idx = self.z.index_from_rc((x, y1));
    self.image.splice(idx*4..(idx+1)*4, [0, 0, 0, 255]);
  }

  pub fn image_buffer(&self) -> Uint8Array {
    unsafe { Uint8Array::view(&self.image) }
  }

  pub fn background_buffer(&self) -> Uint8Array {
    unsafe { Uint8Array::view(&self.overlay_image) }
  }
}
