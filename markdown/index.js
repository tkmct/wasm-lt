const js = import("./wasm-markdown/build/wasm_markdown")

js.then(js => {
  const btn = document.getElementById('parse-btn-wasm')
  const previewArea = document.getElementById('preview')

  btn.addEventListener('click', () => {
    const input = document.getElementById('markdown-text').value
    previewArea.innerHTML = js.parse(input)
  })
});

