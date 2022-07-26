# julia

![julia](./doc/julia.png) ![mandel](./doc/mandel.png)

[Animated Julia sets](https://friarswood.net) and a [zoomable Mandelbrot set](https://friarswood.net/mandel.html), implemented in rust and webassembly.

This version is a barebones reworking of a previous implementation that was beset by node.js vulnerabilities.

Based on the tutorial [here](https://wasmbyexample.dev). Doesn't require node.js or webpack.

## build

```sh
wasm-pack build --target web
```

## run locally

Use a proper web server for prod. For local dev:

```sh
python3 -m http.server
```

## issues

- `export default` in wasm-pack generated js (remove last line and add `init` to the exports in the line above )
- less js, more rust...

## deploy

TODO...