const js = import("./wasm-markdown/wasm_markdown");

js.then(js => {
  js.greet("World!");
});
