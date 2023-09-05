# typst-qrcode-wasm
QR Code plugin for Typst.

## Usage
Download WASM plugin from [Releases](https://github.com/megakite/typst-qrcode-wasm/releases) and place it wherever you want.

Then from Typst:
```
#let typst-qrcode-wasm = plugin("path/to/typst_qrcode_wasm.wasm")
#let qrcode(arg) = image.decode(str(
  typst-qrcode-wasm.generate(bytes(arg))
))
#qrcode("Lorem ipsum")
```
