const js = import("./wasm-markdown/build/wasm_markdown");

js.then(js => {
  console.info(js.parse("Takamichi!"))
});
