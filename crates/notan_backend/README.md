Notan Backend
===

This crate is used by default for the `AppBuilder` to set a backend depending on the build platform.

For `web` it will use the `notan_web` backend based on `web-sys` and for desktop platforms it will use `notan_winit` currently using `glutin` to give the OpenGL context.

Both backends (web and winit) will use `notan_glow` to manage the OpenGL/WebGL context. 