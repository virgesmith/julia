
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

pub fn colour_map(n: usize) -> Vec<Vec<u8>> {
  let mut colours = vec![vec![0u8; 4]; n+1];

  let u = n - 1;
  let t = std::f64::consts::PI / (u as f64);

  for (i, colour) in colours.iter_mut().enumerate().take(n + 1) {
    colour[0] = 255 - (127.5 * (1. - (i as f64 * t).cos())) as u8;
    colour[1] = 255 - (127.5 * (1. - ((i*3) as f64 * t).cos())) as u8;
    colour[2] = 255 - (127.5 * (1. - ((i*5) as f64 * t).cos())) as u8;
    colour[3] = 255;
  }

  colours
}

