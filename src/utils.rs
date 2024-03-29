
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

pub type ImageSize = (u32, u32); // width, height

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[allow(unused_macros)]
macro_rules! log {
  ( $( $t:tt )* ) => {
      web_sys::console::log_1(&format!( $( $t )* ).into());
  }
}

fn intensity(i: usize, m: usize, scale: f64) -> u8 {
  255 - (127.5 * (1. - ((i * m) as f64 * scale).cos())) as u8
}

pub fn colour_map(n: usize, cycles: (usize, usize, usize), alpha: u8) -> Vec<[u8; 4]> {

  let u = n - 1;
  let t = std::f64::consts::PI / (u as f64);

  (0..n).map(|i| [intensity(i, cycles.0, t),
                  intensity(i, cycles.1, t),
                  intensity(i, cycles.2, t),
                  alpha]).collect::<Vec<[u8; 4]>>()
}

