import brotli from "brotli-dec-wasm";

async function main() {
  const { decompress } = await brotli;
  window.decompress = decompress;
}

main().then(() => {
  console.log("init ok");
  document.getElementById("status").innerText = "init ok";
});
