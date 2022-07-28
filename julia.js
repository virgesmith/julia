"use strict";

import init, {Julia} from "./pkg/julia.js";

let animationId = null;

const runWasm = async () => {
  await init("./pkg/julia_bg.wasm");

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

  const canvasContext = canvasElement.getContext("2d");
  const canvasImageData = canvasContext.createImageData(
    canvasElement.width,
    canvasElement.height
  );

  var julia = new Julia(0.1, 0.1, 2.0, canvasElement.width, canvasElement.height);

  const render = () => {

    julia.tick();
    julia.render();

    canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);

    const imageData = julia.image_buffer();

    canvasImageData.data.set(imageData);

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