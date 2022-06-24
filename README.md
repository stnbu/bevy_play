"CubeSat 2D!"
=============

Go to `./index.html` to start pulling the thread.

Welcome to the heart of the galactic empire <sccrrratch sound>

It's a rectangle on the plane looking down. You can adjust it with arrow keys. If it goes out of window, good luck getting it back.

The bigger point is that it's 99% Rust compiled to wasm and hosted with the inescapable ancient legacy garbage (`index.html`).

## How to do

1. Set everything up.
1. cargo build --release --target wasm32-unknown-unknown
1. wasm-bindgen --out-dir pkg --target web --reference-types --no-typescript --omit-default-module-path target/wasm32-unknown-unknown/release/bevy_play.wasm

if `./` is a web server you can go to `/index.html`, which will load the wasm, providing hour of fun.

I sync with:

```
rsync -a index.html pkg mywebserver:/mypath/
```

The wasm file is 16mb, so you might have to blink a few times.
