Renderer
========

This crate provides a graphics API to draw to the screen.

### Graphics
It's a low-level 3D API. The main purpose of this API is to serve as foundation of the `Draw` API.
It aims to be a safe, productive and easy API to use on top of different graphics API. Right now is built on top of [glow.rs](TODO) which provides support for OpenGL, OpenGL ES and WebGL2.
But eventually a new backend using [wgpu.rs](TODO) is planned, keeping the current backend as fallback for some platforms.
The reason that wgpu.rs is not the selected backend right now is because Browser support is one of the main target of this project, and wgpu.rs doesn't 
support it right now. Using glow as a first backend we ensure that we can target a wide range of platforms even old ones.  

### Draw 
It's a high-level 2D API. It's built on top of `Graphics`. The purpose of this API is to provide a fast way
to draw images, primitives, patterns, shaders, etc... on the screen without the hassle of a low-level API:

#### Wishlist
- A new high-level 3D API (similar to three.js or heaps.io/h3d)
