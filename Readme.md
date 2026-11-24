# Welcome to the FRUG repo!

In here you will find the source code for the FRUG library.

## About

FRUG is intended to provide a similar abstraction layer over graphics programming as to how Monogame does for C#, meaning that it should provide developers enough control and flexibility to implement their own architectures & design patterns, yet simplifying the process of working with graphics so developers won't have to worry about implementing all the repetitive tasks related to getting things to the screen.

## Getting started

```bash
cargo add frug
```

You don't need to install SDL. By default FRUG compiles SDL3, SDL3_image and SDL3_ttf from bundled source and links them statically into your game, so there are no DLLs or system libraries to ship. The first build takes a couple of minutes; later builds are cached.

The only requirements are a C compiler and [CMake](https://cmake.org/download/):

- **Windows:** Visual Studio (or its Build Tools) with the "Desktop development with C++" workload, which includes CMake.
- **macOS:** `xcode-select --install` and `brew install cmake`.
- **Linux (Debian/Ubuntu):** a compiler, CMake and the windowing/audio headers SDL uses:
  ```bash
  sudo apt-get install build-essential cmake libasound2-dev libpulse-dev libx11-dev libxext-dev libxrandr-dev libxcursor-dev libxfixes-dev libxi-dev libxss-dev libxtst-dev libxkbcommon-dev libdrm-dev libgbm-dev libgl1-mesa-dev libgles2-mesa-dev libegl1-mesa-dev libwayland-dev libdecor-0-dev libdbus-1-dev libudev-dev
  ```

To link against an SDL3 you installed yourself instead, turn off the default feature:

```toml
frug = { version = "0.3", default-features = false }
```

Try an example from this repo with `cargo run --example pong`.

Please check out [the documentation](https://santyarellano.github.io/frug_book/) to learn more about the project and how to use it! (there is also the cargo documentation, but I'm afraid that part is not as polished for now)

> I'm currently migrating this whole thing to work more like what monogame is for C#. So I'm changing this so instead of using WGPU we'll use SDL3 (rust API) and hopefully I'll find some way to make it work with game consoles as well.

> On a side note, please take into account that the project is in it's first iteration and many features are still under development or missing.