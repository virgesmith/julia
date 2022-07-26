"use strict";

import init, {Mandel} from "./pkg/julia.js";

const runWasm = async () => {
  await init("./pkg/julia_bg.wasm");

  const canvasElement = document.querySelector("canvas");
  const info = document.querySelector("p");

  const canvasContext = canvasElement.getContext("2d");
  const canvasImageData = canvasContext.createImageData(
    canvasElement.width,
    canvasElement.height
  );

  var mandel = new Mandel(canvasElement.width, canvasElement.height, 4096);

  const imageData = mandel.image_buffer();

  canvasImageData.data.set(imageData);

  const updateCoords = () => {
    info.innerHTML = `centre=${mandel.mid_r()}+${mandel.mid_i()}i scale=${mandel.scale()}`;
  };

  const render = () => {

    mandel.render();
    updateCoords();

    canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);

    const imageData = mandel.image_buffer();

    canvasImageData.data.set(imageData);

    canvasContext.putImageData(canvasImageData, 0, 0);
    canvasElement.style.cursor = "zoom-in";
  };

  document.onmousedown = (event) => {
    var factor = 0.5;
    if (event.button == 0) { // left click zooms in
      factor = 2.0;
    } else if (event.button != 1) { // wheel click zooms out, anything else ignored
      return;
    }

    const rect = canvasElement.getBoundingClientRect();

    const x = (event.clientX - rect.left) * canvasElement.width / canvasElement.clientWidth;
    const y = (event.clientY - rect.top) * canvasElement.height / canvasElement.clientHeight;

    if (x >= 0 && y >= 0 && x <= canvasElement.width && y <= canvasElement.height) {
      canvasElement.style.cursor = "progress"; // no discernible effect
      mandel.zoom(y, x, factor);
      render();
    }
  }
  render();
}
runWasm();