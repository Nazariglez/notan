<h1 align="center">Nae</h1>
<div align="center">
 <strong>
   Cross platform multimedia layer made with Rust
 </strong>
</div>

<br />

## About
Nae is Not An Engine, is a portable multimedia layer with an easy API designed to make your own multimedia apps on top of it.

## Goals
- HTML5 must be a first class citizen using Webassembly.
- Support all the major platforms.
- Provide an abstraction layer to add new platforms easily (even privates like consoles). 
- Make easy to deploy on these platforms with a CLI.
- Provide an API that run on each platform without changes.

## Examples
```rust 
use nae::prelude::*;

#[nae::main]
fn main() {
    nae::init().draw(draw).build().unwrap();
}

fn draw(app: &mut App, _: &mut ()) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));
    draw.set_color(Color::GREEN);
    draw.triangle(400.0, 100.0, 100.0, 500.0, 700.0, 500.0);
    draw.end();
}

```
![Triangle](./assets/triangle.png)

**More examples**
- [Triangle](https://github.com/Nazariglez/nae/blob/master/examples/triangle.rs)

## Getting started
Instructions to init and build a project

## Current state
- Targets
    - [x] Web Browsers
    - [ ] iOS
    - [ ] Android
    - [x] MacOS
    - [ ] Linux 
    - [x] Windows
- 2D renderer
    - [x] Primitives
    - [x] Polylines
    - [x] Sprites
    - [x] Patterns
    - [x] Masking
    - [x] Custom Shader
    - [x] Text
    - [x] NineSlice
    - [x] BlendModes
    - [x] Surfaces
- Drivers
    - [ ] WebGL
    - [x] WebGL 2
    - [ ] Metal
    - [ ] Dx11
    - [ ] Dx12
    - [ ] Vulkan
    - [x] OpenGL
    - [ ] OpenGL ES
- API 
    - [ ] Window
    - [ ] Keyboard
    - [ ] Mouse
    - [ ] Touch
    - [ ] Gamepad
    - [ ] Audio
- Extras
    - [ ] PostProcess 
    - [ ] Animations
    - [ ] Particles
    - [ ] Tweens
    - [ ] Atlas
    - [ ] SVGs
    - [ ] BitmapText
    - [ ] UI System (Maybe Iced?)

## License
...

## Contribution
...
