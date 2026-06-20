use wasm_bindgen::prelude::*;

use js_sys::Uint8Array;

use num_complex::Complex as Cplx;
use std::cmp::{max, min};

use crate::argand::ZPlane;
use crate::mandel::Mandel;
use crate::utils::{colour_map, set_panic_hook};

type Cell = u8;

#[wasm_bindgen]
pub struct Julia {
    z: ZPlane<Cell>,
    c: Cplx<f64>, // as in z <-> z*z + c
    a: Cplx<f64>, // attraction point that c moves to
    image: Vec<u8>,
    colour_map: Vec<[u8; 4]>,
    overlay_image: Vec<u8>,
    inside_mandel: Vec<bool>,
}

// speed at which c is pulled to a
const SPEED: f64 = 0.01;

const MAXITER: Cell = 254;
// reduced max iteration when point c is inside the Mandelbrot boundary, for perfomance
const MAXITER_INSIDE: Cell = 63;
// the reduction is only applied this far (in pixels) inside the boundary, so it doesn't kick
// in right at the detailed edge of the set. Larger = the reduction stays further from the edge.
const BUFFER_PIXELS: i32 = 6;
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

        let maxiter = 512;
        let colour_depth = maxiter as usize;
        let mut mandel = Mandel::custom(bottom_left, top_right, (width, height), maxiter, colour_depth, (2, 2, 2), 255);
        mandel.render();

        // points that never escaped are inside the Mandelbrot set...
        let inside = mandel.iterations().iter().map(|x| *x == maxiter - 1).collect::<Vec<bool>>();
        // ...but erode that mask by a buffer so the iteration reduction is imposed a small
        // distance inside the boundary, not right at the detailed edge where it would show
        let inside_mandel = erode(&inside, (width, height), BUFFER_PIXELS);

        let mut julia = Julia {
            z: ZPlane::<Cell>::new(bottom_left, top_right, (width, height)),
            c: Cplx::new(cr, ci),
            a: Cplx::new(0.0, 0.0),
            image: vec![0u8; (width * height * 4) as usize],
            colour_map: colour_map(512, (1, 2, 3), 192),
            overlay_image: mandel.raw_image(),
            // TODO we could just store an array of maxiter (u8)?
            inside_mandel,
        };

        julia.draw();
        julia
    }

    pub fn set_attract(&mut self, row: u32, col: u32) {
        self.a = self.z.point_from_rc((row, col));
    }

    pub fn tick(&mut self) {
        self.c += Cplx::new((self.a.re - self.c.re) * SPEED, (self.a.im - self.c.im) * SPEED);
        if self.c.re > self.z.zmax.re {
            self.c.re = self.z.zmax.re;
        }
        if self.c.re < self.z.zmin.re {
            self.c.re = self.z.zmin.re;
        }
        if self.c.im > self.z.zmax.im {
            self.c.im = self.z.zmax.im;
        }
        if self.c.im < self.z.zmin.im {
            self.c.im = self.z.zmin.im;
        }

        self.draw();
    }

    fn iterate(&self, i: usize, maxiter: Cell) -> Cell {
        let mut z = self.z.point_from_index(i);
        let mut iter: Cell = 0;
        let mut r2 = z.re * z.re;
        let mut i2 = z.im * z.im;
        while r2 + i2 < self.z.scale.re /*400.*/ && iter < maxiter {
            // z = z * z + self.c;
            z.im = (z.re + z.re) * z.im + self.c.im;
            z.re = r2 - i2 + self.c.re;
            r2 = z.re * z.re;
            i2 = z.im * z.im;
            iter += ITER_INC;
        }
        // if reduced iterations return MAXITER anyway so that colours are correct
        match iter {
            i if i >= maxiter => MAXITER,
            _ => iter,
        }
    }

    fn draw(&mut self) {
        // iterate over half the plane (symmetry)
        let n = (self.z.size.0 * self.z.size.1 / 2) as usize;
        let mut next = vec![0 as Cell; n];

        // when c is inside the Mandelbrot set, iterations do not diverge, so reduce the max iterations
        // for performance (the Julia sets are less interesting here anyway)
        let maxiter = match self.inside_mandel[self.z.index_from_point(&self.c)] {
            true => MAXITER_INSIDE,
            _ => MAXITER,
        };

        next.iter_mut().enumerate().for_each(|(i, n)| *n = self.iterate(i, maxiter));
        self.z.cells[0..n].copy_from_slice(&next);
        next.reverse();
        self.z.cells[n..].copy_from_slice(&next);
    }

    pub fn render(&mut self) {
        self.image = (0..(self.z.size.0 * self.z.size.1) as usize)
            .flat_map(|i| self.colour_map[self.z.cells[i] as usize])
            .collect::<Vec<_>>();

        // plot the locus
        let (x, y) = self.z.rc_from_point(&self.c);
        let idx = self.z.index_from_rc((x, y));
        self.image.splice(idx * 4..(idx + 1) * 4, [0, 0, 0, 255]);

        let x1 = min(x + 1, self.z.size.0 - 1);
        let idx = self.z.index_from_rc((x1, y));
        self.image.splice(idx * 4..(idx + 1) * 4, [0, 0, 0, 255]);
        let x1 = max(x - 1, 0);
        let idx = self.z.index_from_rc((x1, y));
        self.image.splice(idx * 4..(idx + 1) * 4, [0, 0, 0, 255]);

        let y1 = max(y - 1, 0);
        let idx = self.z.index_from_rc((x, y1));
        self.image.splice(idx * 4..(idx + 1) * 4, [0, 0, 0, 255]);

        let y1 = min(y + 1, self.z.size.1 - 1);
        let idx = self.z.index_from_rc((x, y1));
        self.image.splice(idx * 4..(idx + 1) * 4, [0, 0, 0, 255]);
    }

    pub fn image_buffer(&self) -> Uint8Array {
        unsafe { Uint8Array::view(&self.image) }
    }

    pub fn background_buffer(&self) -> Uint8Array {
        unsafe { Uint8Array::view(&self.overlay_image) }
    }
}

// Erode a boolean mask by `buffer` pixels (square structuring element): a cell stays `true`
// only if every cell within `buffer` of it - including off-grid cells, treated as `false` - is
// also `true`. Done as two separable 1D passes, so O(width * height * buffer), run once at
// construction. Used to pull the "inside Mandelbrot" region back from the boundary.
fn erode(mask: &[bool], (width, height): (u32, u32), buffer: i32) -> Vec<bool> {
    let (w, h) = (width as i32, height as i32);
    let idx = |r: i32, c: i32| (r * w + c) as usize;
    let window_all = |get: &dyn Fn(i32) -> bool, p: i32, lim: i32| {
        (-buffer..=buffer).all(|d| {
            let q = p + d;
            q >= 0 && q < lim && get(q)
        })
    };

    let mut horiz = vec![false; mask.len()];
    for r in 0..h {
        for c in 0..w {
            horiz[idx(r, c)] = window_all(&|cc| mask[idx(r, cc)], c, w);
        }
    }

    let mut out = vec![false; mask.len()];
    for c in 0..w {
        for r in 0..h {
            out[idx(r, c)] = window_all(&|rr| horiz[idx(rr, c)], r, h);
        }
    }
    out
}
