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

  var mandel = new Mandel(canvasElement.width, canvasElement.height, 2048);

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
  };


  (function() {
    "use strict";

    document.onmousedown = handleMouseClick;
    function handleMouseClick(event) {

      var factor = 0.5;
      if (event.which == 1) { // left click zooms in
        factor = 2.0;
      } else if (event.which != 2) { // wheel click zooms out
        return;
      }

      const rect = canvasElement.getBoundingClientRect();

      const x = (event.clientX - rect.left) * canvasElement.width / canvasElement.clientWidth;
      const y = (event.clientY - rect.top) * canvasElement.height / canvasElement.clientHeight;

      if (x >= 0 && y >= 0 && x <= canvasElement.width && y <= canvasElement.height) {
        mandel.zoom(x, y, factor);
        render();
      }
    }
  })();

  render();
};

runWasm();