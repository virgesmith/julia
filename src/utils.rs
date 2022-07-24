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


// export function getColours(n) {
//   var colours = [];
//   const u = n - 1;
//   const s = u / 2;
//   const t = Math.PI/u;
//   for (var i=0; i < n; ++i) {
//     var r = 255 - Math.round(127.5 * (1.0 - Math.cos(i *     t)));
//     var g = 255 - Math.round(127.5 * (1.0 - Math.cos(i * 3 * t)));
//     var b = 255 - Math.round(127.5 * (1.0 - Math.cos(i * 5 * t)));

//     colours.push(colour(r, g, b));
//   }
//   return colours;
// };