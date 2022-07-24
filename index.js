import {init, Julia, Mandel} from "./pkg/julia.js";
//import Julia from "./pkg/julia.js";

const runWasm = async () => {
  // Instantiate our wasm module
  const rustWasm = await init("./pkg/julia_bg.wasm");
  //const rustWasm = initSync("./pkg/julia_bg.wasm");

  // Get our canvas element from our index.html
  const canvasElement = document.querySelector("canvas");

  // Set up Context and ImageData on the canvas
  const canvasContext = canvasElement.getContext("2d");
  const canvasImageData = canvasContext.createImageData(
    canvasElement.width,
    canvasElement.height
  );

  // Clear the canvas
  canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);

  // var julia = new Julia(0., 0., 2.0, canvasElement.width, canvasElement.height);
  var julia = new Mandel(canvasElement.width, canvasElement.height, 255);
  julia.render();

  const imageData = julia.image_buffer();

  // Set the values to the canvas image data
  canvasImageData.data.set(imageData);

  const render = () => {
    //julia.tick();

    // const imageData = julia.cells();

    // // Set the values to the canvas image data
    // canvasImageData.data.set(imageData);

    // Clear the canvas
    canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);

    const imageData = julia.image_buffer();

    // Set the values to the canvas image data
    canvasImageData.data.set(imageData);

    // Place the new image onto the canvas
    canvasContext.putImageData(canvasImageData, 0, 0);
    console.log("rendered");
  };

  render();
  setInterval(() => {
    render();
  }, 1000);
};
runWasm();