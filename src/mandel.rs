use wasm_bindgen::prelude::*;

use num_complex::Complex as Cplx;
use js_sys::Uint8Array;

use crate::utils::{colour_map, set_panic_hook};
use crate::argand::ZPlane;

type Cell = u16;

const COLOUR_MAP_SIZE: usize = 1024;

#[wasm_bindgen]
pub struct Mandel {
  z: ZPlane<Cell>,
  depth: Cell,
  image: Vec<u8>,
  colour_map: Vec<[u8; 4]>
}

impl Mandel {
  pub fn custom(bottom_left: Cplx<f64>, top_right: Cplx<f64>, width: u32, height: u32,
    maxiter: Cell, cmap_size: usize, cmap_cycles: (usize, usize, usize), alpha: u8) -> Mandel {
    let mut mandel = Mandel {
      z: ZPlane::<Cell>::new(bottom_left, top_right, width, height),
      depth: maxiter,
      image: vec![0u8; (width * height * 4) as usize],
      colour_map: colour_map(cmap_size, cmap_cycles, alpha)
    };

    mandel.draw();
    mandel
  }
}

#[wasm_bindgen]
impl Mandel {

  #[wasm_bindgen(constructor)]
  pub fn new(width: u32, height: u32, maxiter: Cell) -> Mandel {
    set_panic_hook();

    // fit to width and adjust y scale according to height
    let ylim = 1.25 * height as f64 / width as f64;

    let bottom_left = Cplx::<f64>::new(-2.0, -ylim);
    let top_right = Cplx::<f64>::new(0.5, ylim);

    let mut mandel = Mandel {
      z: ZPlane::<Cell>::new(bottom_left, top_right, width, height),
      depth: maxiter,
      image: vec![0u8; (width * height * 4) as usize],
      colour_map: colour_map(COLOUR_MAP_SIZE, (3, 3, 1), 255)
    };

    mandel.draw();
    mandel
  }

  pub fn iterations(&self) -> Vec<Cell> {
    self.z.cells.clone()
  }

  pub fn raw_image(&self) -> Vec<u8> {
    self.image.clone()
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
    let c = self.z.point_from_rc((row, col));
    self.z.scale *= factor;
    let dr = (self.z.zmax.re - self.z.zmin.re) / (2.0 * factor);
    let di = (self.z.zmax.im - self.z.zmin.im) / (2.0 * factor);
    self.z.zmin.re = c.re - dr;
    self.z.zmin.im = c.im - di;
    self.z.zmax.re = c.re + dr;
    self.z.zmax.im = c.im + di;
    self.draw();
  }

  fn draw(&mut self) {
    let n = (self.z.width * self.z.height) as usize;
    let mut next = vec![0 as Cell; n];

    next.iter_mut().enumerate().for_each(|(i, n)| *n = self.iterate(i));
    self.z.cells.copy_from_slice(&next);
  }

  fn iterate(&self, i: usize) -> Cell {
    let c = self.z.point_from_index(i);
    let mut z = Cplx::new(0.0, 0.0);
    let mut it: Cell = 0;
    let mut r2 = 0.0;
    let mut i2 = 0.0;
    let mut z_prev = z;
    let mut period = 0;
    while it < self.depth - 1 && (r2 + i2) < self.z.scale.re {
      // z <- z * z + c;
      // hand optimised
      z.im = (z.re + z.re) * z.im + c.im;
      z.re = r2 - i2 + c.re;
      r2 = z.re * z.re;
      i2 = z.im * z.im;
      it += 1;
      // check for periodicity and break early
      if z == z_prev {
        it = self.depth - 1;
        break;
      }
      period += 1;
      // update ref value every 20th iteration (can detect cycles up to 20 iterations long)
      if period > 20 {
        period = 0;
        z_prev = z;
      }
    }
    it
  }

  // TODO cleverer mapping between iters and cmap...logarithmic?
  pub fn render(&mut self) {
    self.image = (0..(self.z.width * self.z.height) as usize)
      .flat_map(|i| self.colour_map[self.z.cells[i] as usize % COLOUR_MAP_SIZE])
      .collect::<Vec<_>>();
  }

  pub fn image_buffer(&self) -> Uint8Array {
    unsafe { Uint8Array::view(&self.image) }
  }
}
