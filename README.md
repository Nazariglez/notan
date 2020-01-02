<h1 align="center">Nae</h1>
<div align="center">
 <strong>
   Cross platform multimedia layer made with Rust
 </strong>
</div>

<br />

## About
Nae is a portable multimedia layer designed to make your own multimedia apps on top of it with a mid-level and easy to use API.

## Goals
The main goal of Nae is to be the foundation for cross-platform multimedia applications, game engines or games while keeping the user code simple
and free as much as possible of "platform" dependent code. This means, that you can write your code once, and export to multiple targets without changes.

### More goals
- The Web is treated as a first class citizen, you can export to Webassembly your apps easily with Nae. We use (web_sys and wasm_bindgen).
- Support the major platforms at this moment. (Web, MacOS, Windows, Linux, Android, iOS)
- The structure of the lib makes relative easy to add new platforms just replacing the backend dependency. (Maybe will be useful for consoles eventually)
- Provide a basic set of features and also some extras as optional dependency 

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

## What means Nae?
Nae stands for: Nae is `Not An Engine`. This reference that Nae is not a raw low-level lib nor a high-level game/app engine, is more like a mid-layer 
that can be used as a foundation for this purpose. 

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
