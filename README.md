# gif-extract

Extracts a given .gif file's frames and saves them as .png files under a directory.

## Building

With the Rust toolchain installed, run `cargo build --release` and grab the compiled executable in the target/release folder.

## How to use

To use gif-extract, open a terminal window on the same directory as the executable and run:

```bash
gif-extract <GIF path>
```

gif-extract will then create a new directory under the same path as the gif with name "gifname-out" (gifname being the name of the GIF file minus extension), failing if that directory already exists. This program outputs one 8-bit depth PNG image per GIF frame.
