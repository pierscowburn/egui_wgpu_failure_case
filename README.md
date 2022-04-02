# Description

This repo contains a minimal reproducible example of egui running in the browser via wgpu. Currently this
works successfully when using `wgpu::Backends::BROWSER_WEBGPU` in Chrome Canary or Firefox Nightly with WebGPU
enabled, but does not work when using `wgpu::Backends::GL`.

The example can be run as follows:

```bash
# Run with wgpu::Backends::BROWSER_WEBGPU
RUSTFLAGS=--cfg=web_sys_unstable_apis cargo run-wasm example

# Run with wgpu::Backends::GL
cargo run-wasm example --features webgl
```

A ticket for this issue can be found at [gfx-rs/wgpu/issues/2573](https://github.com/gfx-rs/wgpu/issues/2573).

## Expected behaviour

The example should render with a green background when using either backend.

## Observed behaviour

With the `wgpu::Backends::BROWSER_WEBGPU` backend the example renders correctly with a green background.

With the `wgpu::Backends::GL` backend the example renders with a red background (the clear color is set to red),
and some WebGL errors are printed to the console.

In Canary the warning is:

```
[.WebGL-0x7015b47700] GL_INVALID_OPERATION: It is undefined behaviour to use a uniform buffer that is too small.
```

In Firefox the warnings are:

```
WebGL warning: drawElementsInstanced: Buffer for uniform block is smaller than UNIFORM_BLOCK_DATA_SIZE.
WebGL warning: getSyncParameter: ClientWaitSync must return TIMEOUT_EXPIRED until control has returned to the user agent's main loop. (only warns once) 
```