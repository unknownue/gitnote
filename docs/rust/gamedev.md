# Awesome Rust GameDev

[![Awesome](https://awesome.re/badge-flat2.svg)](https://awesome.re)

## Contents

- [Reference](#Reference)
- [Rendering](#Rendering)
- [Window](#Window)
- [Math](#Math)
- [GUI](#GUI)
- [ECS](#ECS)
- [Engine](#Engine)
- [Texture/Font](#Texture/Font)
- [Design](#Design)
- [Network](#Network)
- [Performance](#Performance)
- [Language Extensions](#Language Extensions)
- [Games](#Games)
- [Emulator](#Emulator)



## Reference

[Rust Official Gamedev Group](https://rust-gamedev.github.io/)

[Are we game yet](https://rust-gamedev.github.io/)



## Rendering

| **Framework**                                        | Description                                                  |
| ---------------------------------------------------- | ------------------------------------------------------------ |
| [luminance](https://github.com/phaazon/luminance-rs) | Type-safe, type-level and stateless Rust graphics framework. |
| [raylib-rs](https://github.com/deltaphc/raylib-rs)   | Rust binding for [raylib](http://www.raylib.com/) 2.0.       |
| [pixels](https://github.com/parasyte/pixels)         | A tiny hardware-accelerated pixel frame buffer.              |
| [euc](https://github.com/zesterer/euc)               | A software rendering crate that lets you write shaders with Rust. |

See also https://stevenlr.com/posts/handmade-rust-4-vulkan-bindings/



## Window

| Library                                            | Description                                                  |
| -------------------------------------------------- | ------------------------------------------------------------ |
| [gilrs](https://gitlab.com/gilrs-project/gilrs)    | Game Input Library for Rust.                                 |
| [beryllium](https://github.com/Lokathor/beryllium) | An opinionated set of high level wrappers for the `fermium` SDL2 bindings. |



## Math

| Library                                              | SIMD Support | Description                                                  |
| ---------------------------------------------------- | ------------ | ------------------------------------------------------------ |
| [mint](https://github.com/kvark/mint)                | -            | Provides standard mathematical types used in computer graphics. |
| [glam-rs](https://github.com/bitshifter/glam-rs)     | Yes          | A simple and fast linear algebra library for games and graphics. |
| [vek](https://github.com/yoanlcq/vek)                | Yes          | Generic 2D-3D math swiss army knife for game engines, with SIMD support and focus on convenience. |
| [sdfu](https://github.com/termhn/sdfu/)              | -            | Signed Distance Field Utilities.                             |
| [ultraviolet](https://github.com/termhn/ultraviolet) | Yes          | A wide linear algebra crate taking full advantage of SIMD.   |

Note: See [mathbench-rs](https://github.com/bitshifter/mathbench-rs) for benchmarks comparation of various linear algebra libraries.



## GUI

| Library                                                   | is binding? | Description                                                  |
| --------------------------------------------------------- | ----------- | ------------------------------------------------------------ |
| [iced](https://github.com/hecrj/iced)                     | No          | A renderer-agnostic GUI library for Rust focused on simplicity and type-safety. |
| [imgui-rs](https://github.com/Gekkio/imgui-rs)            | Yes         | Rust bindings for dear imgui.                                |
| [imgui-inspect](https://github.com/aclysma/imgui-inspect) | Yes         | An inspector UI using imgui in Rust.                         |
| [kas](https://github.com/dhardy/kas)                      | No          | KAS is a general-purpose GUI toolkit.                        |



## ECS

| Library                                                      | Description                                                  | Websize                    |
| ------------------------------------------------------------ | ------------------------------------------------------------ | -------------------------- |
| [Legion](https://github.com/TomGillen/legion)                | High performance Rust ECS library.                           |                            |
| [Specs](https://github.com/amethyst/specs)                   | Specs is an Entity-Component System written in Rust.         | https://specs.amethyst.rs/ |
| [component_group](https://github.com/sunjay/component_group) | A Rust crate for working with a group of Components (in the Specs ECS) |                            |



## Engine

| Engine                                           | Description                                                  | Official Website     |
| ------------------------------------------------ | ------------------------------------------------------------ | -------------------- |
| [amethyst](https://github.com/amethyst/amethyst) | Amethyst is a data-driven and data-oriented game engine aiming to be fast and as configurable as possible. | https://amethyst.rs/ |
| [rg3d](https://github.com/mrDIMAS/rg3d)          | Yet another 3d game engine written in Rust.                  |                      |



## Physics

| Library                                   | Description                                           | Official Website  |
| ----------------------------------------- | ----------------------------------------------------- | ----------------- |
| [salva](https://github.com/rustsim/salva) | 2 and 3-dimensional fluid simulation library in Rust. | https://salva.rs/ |





## Texture/Font

| Library                                                      | Description                                                  |
| ------------------------------------------------------------ | ------------------------------------------------------------ |
| [texture-synthesis](https://github.com/EmbarkStudios/texture-synthesis) | A light Rust API for *Multiresolution Stochastic Texture Synthesis*. |
| [Fontdue](https://github.com/mooman219/fontdue)              | A simple no_std font parser and rasterizer.                  |
| [blurhash-rs](https://github.com/Raincal/blurhash-rs)        | Encode an image into a short ASCII string.                   |



## Design

| Library                                     | Description                                                  |
| ------------------------------------------- | ------------------------------------------------------------ |
| [VeoLuz](https://github.com/jaredly/veoluz) | Visualize the paths of millions of light rays through reflection, refraction and diffusion. |
| [lyon](https://github.com/nical/lyon)       | 2D graphics rendering on the GPU in rust using path tessellation. |



## Network

| Library                                            | Description                                                  | Websize              |
| -------------------------------------------------- | ------------------------------------------------------------ | -------------------- |
| [Ruma](https://github.com/ruma/ruma)               | Ruma is a Matrix homeserver, client, and supporting libraries written in Rust. | https://www.ruma.io/ |
| [Sonant-rs](https://github.com/parasyte/sonant-rs) | A Rust port of the [Sonant 4K synth](http://www.pouet.net/prod.php?which=53615) with streaming support |                      |



## Performance

| Library                                                      | Description                                                  | Website                      |
| ------------------------------------------------------------ | ------------------------------------------------------------ | ---------------------------- |
| [microprofile-rust](https://github.com/jonasmr/microprofile-rust) | A profiler for profiling and optimizing multithreaded game code. |                              |
| [flame](https://github.com/TyOverby/flame)                   | A cool flamegraph library for rust.                          |                              |
| [superluminal-perf-rs](https://github.com/EmbarkStudios/superluminal-perf-rs) | Superluminal Performance profiler Rust API for adding user events to captures. | https://superluminal.eu/rust |



## Language extensions

| Library                                                      | Description                                                  |
| ------------------------------------------------------------ | ------------------------------------------------------------ |
| [mun](https://github.com/mun-lang/mun)                       | Mun is a programming language empowering creation through iteration. [[HomePage]](https://mun-lang.org/) |
| [static-assertions-rs](https://github.com/nvzqz/static-assertions-rs) | Rust compile-time assertions to ensure that invariants are met. |
| [cmd_lib](https://github.com/rust-shell-script/rust_cmd_lib) | Common rust command line macros and utils, to write shell script like tasks easily in Rust. |
| [anyhow](https://github.com/dtolnay/anyhow)                  | This library provides `anyhow::Error`, a trait object based error type for easy idiomatic error handling in Rust applications. |
| [staticvec](https://github.com/slightlyoutofphase/staticvec) | The staticvec implements a fixed-capacity stack-allocated Vec alternative backed by an array, using const generics. |
| [heapless](https://github.com/japaric/heapless)              | This library implements static friendly data structures that don't require dynamic memory allocation. |
| [confy](https://github.com/rust-cli/confy)                   | Zero-boilerplate configuration management.|



## Games

| Games                                                       | Description                                                  |
| ----------------------------------------------------------- | ------------------------------------------------------------ |
| [klondike-rs](https://github.com/chrisbouchard/klondike-rs) | CLI Klondike Solitaire written in Rust                       |
| [skulpin](https://github.com/aclysma/skulpin)               | Drawing hardware-accelerated 2D by combining vulkan and skia. |



## Emulator

| Library                                     | Description               |
| ------------------------------------------- | ------------------------- |
| [nestur](https://github.com/spieglt/nestur) | Nestur is an NES emulator |





## Service

|                                                      | Description                        |
| ---------------------------------------------------- | ---------------------------------- |
| [igdb-rs](https://github.com/CarlosLanderas/igdb-rs) | Non-Official IGDB Rust Api Client. |



## Resources

[Vulkan Samples](https://github.com/khronosGroup/Vulkan-samples) -- One stop solution for all Vulkan samples

[Vulkan Raytracing Tutorial](https://github.com/nvpro-samples/vk_raytracing_tutorial) -- Raytracing Tutorial from NVIDA
