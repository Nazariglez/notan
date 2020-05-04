Renderer
========

This crate provides a graphics API to draw to the screen.

### Graphics
It's a low-level 3D API. The purpose of this API is to serve as foundation of the `Draw` API.
Aims to be a productive and easy API to use. Built on top of [glow.rs](TODO) which provides support for OpenGL, OpenGL ES and WebGL2.

Eventually a new backend using [wgpu.rs](TODO) is planned, but keeping the current backend as fallback for some platforms. 

### Draw 
It's a high-level 2D API. It's built on top of `Graphics`. The purpose of this API is to provide a fast way
to draw images, primitives, patterns, shaders, etc... on the screen without the hassle of a low-level API:

