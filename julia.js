"use strict";

import {init, Julia} from "./pkg/julia.js";

const CELL_SIZE = 1;

let animationId = null;

const runWasm = async () => {
  // Instantiate our wasm module
  const rustWasm = await init("./pkg/julia_bg.wasm");

  // Get our canvas element from our index.html
  const canvasElement = document.querySelector("canvas");
  const playPauseButton = document.querySelector("button");

  const isPaused = () => {
    return animationId === null;
  };

  const play = () => {
    playPauseButton.textContent = "⏸";
    render();
  };

  const pause = () => {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
  };

  playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
      play();
    } else {
      pause();
    }
  });


  // Set up Context and ImageData on the canvas
  const canvasContext = canvasElement.getContext("2d");
  const canvasImageData = canvasContext.createImageData(
    canvasElement.width,
    canvasElement.height
  );

  var julia = new Julia(0.1, 0.1, 2.0, canvasElement.width, canvasElement.height);

  const render = () => {

    julia.tick();
    julia.render();

    // Clear the canvas
    canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);

    const imageData = julia.image_buffer();

    // Set the values to the canvas image data
    canvasImageData.data.set(imageData);

    // Place the new image onto the canvas
    canvasContext.putImageData(canvasImageData, 0, 0);

    animationId = requestAnimationFrame(render);
  };

  (function() {

    document.onmousemove = handleMouseMove;
    function handleMouseMove(event) {

      const rect = canvasElement.getBoundingClientRect();

      const x = (event.clientX - rect.left) * canvasElement.width / canvasElement.clientWidth;
      const y = (event.clientY - rect.top) * canvasElement.height / canvasElement.clientHeight;

      if (x >= 0 && y >= 0 && x <= canvasElement.width && y <= canvasElement.height) {
        julia.set_attract(x, y);
      }
    }
  })();

  render();
};
runWasm();