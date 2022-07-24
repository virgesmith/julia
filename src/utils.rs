// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;



pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}


// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[allow(unused_macros)]
macro_rules! log {
  ( $( $t:tt )* ) => {
      web_sys::console::log_1(&format!( $( $t )* ).into());
  }
}


// need to copy this from rand because the crate links to C++ static libs
pub struct LCG {
  /// The seed
  r: u32
}

impl LCG {
  const A: u64 = 48271;
  const M: u64 = std::i32::MAX as u64;

  pub fn new(seed: u32) -> LCG {
    assert_ne!(seed, 0);
    LCG{r: seed}
  }

  pub fn next_1(&mut self) -> u32 {
    self.r = ((self.r as u64 * LCG::A) % LCG::M) as u32;
    self.r
  }
}


pub fn colour_map(n: usize) -> Vec<Vec<u8>> {
  let mut colours = vec![vec![0u8; 4]; n+1];

  let u = n - 1;
  let t = std::f64::consts::PI / (u as f64);

  for i in 0..=n {
    colours[i][0] = 255 - (127.5 * (1. - (i as f64 * t).cos())) as u8;
    colours[i][1] = 255 - (127.5 * (1. - ((i*3) as f64 * t).cos())) as u8;
    colours[i][2] = 255 - (127.5 * (1. - ((i*5) as f64 * t).cos())) as u8;
    colours[i][3] = 255;
  }
  colours
}

