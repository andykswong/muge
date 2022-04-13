import { set_context_memory } from 'mugl/wasm';
import { app_id, memory, init, render, resize, destroy } from 'examples.wasm';

const EXAMPLES = [
  'basic',
  'instancing',
  'stencil',
]

set_context_memory(app_id(), memory);

const canvas = document.querySelector('canvas');
const dpr = window.devicePixelRatio || 1;
if (dpr > 1) {
  const { width, height } = canvas;
  canvas.width = width * dpr;
  canvas.height = height * dpr;
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;
}

let raf = 0;
function renderLoop(now = 0) {
  raf = requestAnimationFrame(renderLoop);
  if (!render(now / 1000)) {
    cancelAnimationFrame(raf);
    raf = 0;
  }
}

window.loadExample = function (hash = location.hash) {
  const nextExample = EXAMPLES.indexOf(hash.replace('#', ''));
  if (nextExample >= 0) {
    destroy();
    init(nextExample);
    if (!raf) {
      renderLoop();
    }
  }
};

window.loadExample(location.hash = location.hash || `#${EXAMPLES[0]}`);
