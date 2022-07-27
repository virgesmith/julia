import init, {Mandel} from "./pkg/julia.js";

const CELL_SIZE = 1;

const runWasm = async () => {
  // Instantiate our wasm module
  const rustWasm = await init("./pkg/julia_bg.wasm");

  // Get our canvas element from our index.html
  const canvasElement = document.querySelector("canvas");
  const info = document.querySelector("p");

  // Set up Context and ImageData on the canvas
  const canvasContext = canvasElement.getContext("2d");
  const canvasImageData = canvasContext.createImageData(
    canvasElement.width,
    canvasElement.height
  );

  // var julia = new Julia(0., 0., 2.0, canvasElement.width, canvasElement.height);
  var mandel = new Mandel(canvasElement.width, canvasElement.height, 2048);

  const imageData = mandel.image_buffer();

  // Set the values to the canvas image data
  canvasImageData.data.set(imageData);

  const updateCoords = () => {
    info.innerHTML = `centre=${mandel.mid_r()}+${mandel.mid_i()}i scale=${mandel.scale()}`;
  };

  const render = () => {

    //mandel.tick();
    mandel.render();
    updateCoords();

    // Clear the canvas
    canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);

    const imageData = mandel.image_buffer();

    console.log(imageData);

    // Set the values to the canvas image data
    canvasImageData.data.set(imageData);

    // Place the new image onto the canvas
    canvasContext.putImageData(canvasImageData, 0, 0);
    console.log("rendered");
  };


  (function() {
    "use strict";

    document.onmousedown = handleMouseClick;
    function handleMouseClick(event) {

      const rect = canvasElement.getBoundingClientRect();

      const x = (event.clientX - rect.left) * canvasElement.width / canvasElement.clientWidth;
      const y = (event.clientY - rect.top) * canvasElement.height / canvasElement.clientHeight;

      console.log(x,y);

      //console.log(x, y);
      if (x >= 0 && y >= 0 && x <= canvasElement.width && y <= canvasElement.height) {
        mandel.zoom(x, y);
        render();
      }
    }
  })();

  render();
  // setInterval(() => {
  //   render();
  // }, 1000);
};
runWasm();