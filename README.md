<h1 align="center">Notan</h1>
<div align="center">
 <strong>
     Portable Multimedia Layer
 </strong>
<br />
<br />

<a href="https://gfycat.com/horriblefearlessinvisiblerail">
  <img src="https://thumbs.gfycat.com/HorribleFearlessInvisiblerail-small.gif" height="150px">
</a>
<a href="https://gfycat.com/gregariousgreedycrocodile">
  <img src="https://thumbs.gfycat.com/GregariousGreedyCrocodile-small.gif" height="150px">
</a>
<a href="https://gfycat.com/wealthyseparatefoxhound">
  <img src="https://thumbs.gfycat.com/WealthySeparateFoxhound-small.gif" height="150px">
</a>
<a href="https://gfycat.com/illfatedmisguidedlice">
  <img src="https://thumbs.gfycat.com/IllfatedMisguidedLice-small.gif" height="150px">
</a>

</div>

<br />

__Notan__ is a simple and portable layer, designed to make your own multimedia apps on top of it
without worrying too much about platform-specific code.

The main goal is to provide a set of APIs and tools that can be used to create your project in an ergonomic manner without
enforcing any structure or pattern, always trying to stay out of your way.

## Community 

- [Discord](https://discord.gg/rH3nP7neeu): Join us!

## Examples

* [Online demos](https://nazariglez.github.io/notan-web/).

#### Do you want to open a window?

[Window Open](https://nazariglez.github.io/notan-web/examples/window_open.html)

```rust
use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init().build()
}
```

#### Do you want to draw a triangle?

[Draw Triangle](https://nazariglez.github.io/notan-web/examples/draw_triangle.html)

```rust
use notan::prelude::*;
use notan::draw::*;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init().draw(draw)
        .add_config(DrawConfig)
        .build()
}

fn draw(gfx: &mut Graphics) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    draw.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0));
    gfx.render(&draw);
}
```

#### How about render the triangle directly?

[Renderer Triangle](https://nazariglez.github.io/notan-web/examples/renderer_triangle.html)

```rust
use notan::prelude::*;

//language=glsl
const VERT: ShaderSource = notan::vertex_shader! {
    r#"
    #version 450
    layout(location = 0) in vec2 a_pos;
    layout(location = 1) in vec3 a_color;

    layout(location = 0) out vec3 v_color;

    void main() {
        v_color = a_color;
        gl_Position = vec4(a_pos - 0.5, 0.0, 1.0);
    }
    "#
};

//language=glsl
const FRAG: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) in vec3 v_color;
    layout(location = 0) out vec4 color;

    void main() {
        color = vec4(v_color, 1.0);
    }
    "#
};

#[derive(AppState)]
struct State {
    clear_options: ClearOptions,
    pipeline: Pipeline,
    vbo: Buffer,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup).draw(draw).build()
}

fn setup(gfx: &mut Graphics) -> State {
    let clear_options = ClearOptions::color(Color::new(0.1, 0.2, 0.3, 1.0));

    let vertex_info = VertexInfo::new()
        .attr(0, VertexFormat::Float32x2)
        .attr(1, VertexFormat::Float32x3);

    let pipeline = gfx
        .create_pipeline()
        .from(&VERT, &FRAG)
        .with_vertex_info(&vertex_info)
        .build()
        .unwrap();

    #[rustfmt::skip]
        let vertices = [
        0.5, 1.0,   1.0, 0.2, 0.3,
        0.0, 0.0,   0.1, 1.0, 0.3,
        1.0, 0.0,   0.1, 0.2, 1.0,
    ];

    let vbo = gfx
        .create_vertex_buffer()
        .with_info(&vertex_info)
        .with_data(&vertices)
        .build()
        .unwrap();

    State {
        clear_options,
        pipeline,
        vbo,
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut renderer = gfx.create_renderer();

    renderer.begin(Some(&state.clear_options));
    renderer.set_pipeline(&state.pipeline);
    renderer.bind_buffer(&state.vbo);
    renderer.draw(0, 3);
    renderer.end();

    gfx.render(&renderer);
}
```

#### Looking for more examples?

Sure! Check the [examples folder](examples). You will find there a few of them for any matter, like rendering, windowing, input, etc...

## Installation

Add `notan` to your project from [crates.io](https://crates.io). The `main` branch should always be the latest version on `crates.io`.

## WebAssembly

We treat the web as a first class citizen. Our web backend is made using [web-sys](https://github.com/rustwasm/wasm-bindgen/tree/master/crates/web-sys) and you can compile it using [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)
or [wasm-pack](https://github.com/rustwasm/wasm-pack). Take in account that you need to create an `index.html` file and call the main function from the `wasm` module.

Using `wasm-pack build --release --target web` you need to load the module with something similar to this:

```html

<html>
<head>
    <title>Notan App</title>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport"
          content="minimal-ui, width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
    <meta name="apple-mobile-web-app-capable" content="yes">
</head>
<body>
<script type="module">
    import init from './pkg/YOUR_PROJECT_NAME.js'; // replace this 
    let module = await init();
    module.notan_main();
</script>
<div id="examples">
    <canvas id="notan_canvas"></canvas>
</div>
</body>
```

## How it works

Is quite simple, **Notan** defines a set of APIs for different things (like windows, graphics, inputs, audio, etc...) as a "core".
Below this "core" exists the `backends`, which are crates that add support for any platform managing the platform-specific
code and translating it to our "core" APIs. Anybody should be able to create and use a custom backend easily.
And, on top of the "core" we can build more ergonomic APIs that are usable with any backend made for Notan.

Then the final user only needs to worrying to build their apps on top of Notan and it should be fine no matter the compilation target.
Of course, there are still things to have in mind if you want to target different targets, writing portable code can be tricky sometimes
but Notan should help with most of them.

## Supported platforms

- Web Browsers (`wasm32`) - WebGL2
- Window - OpenGL 3.3
- MacOS - OpenGL 3.3
- Linux - OpenGL 3.3

The current graphics backend in place for these platforms is using [glow.rs](https://github.com/grovesNL/glow) which allow us to target WebGl2, GL and GL ES easily.

## Performance

People love to see performance numbers and benchmarks (I love it too), but the truth is that any
benchmark or numbers worth nothing without the full context of how these numbers were calculated.

We didn't check (*yet*) about which parts of the code can be changed to improve the performance.
Is not an easy task to keep in balance a good API, small boilerplate, and been performant.
However, this is something that we try to accomplish with this project since the idea was born.

**Notan** try to give to the user a simple to build things, and the performance will depend on a lot of factors, user's code included.

Let's see a simple example, the 2D Draw API is built on top of the Graphics API, it has plenty of room for improvements,
but I got some decent numbers on my machine running the example [draw_bunnymark](examples/draw_bunnymark.rs).

On a Macbook (2.3Hz i9 - 16GB RAM):
- Native: 85000 Bunnies at 60FPS
- Chrome: 78000 Bunnies at 60FPS

On a high-end Desktop with Archlinux:
- Native: 205000 Bunnies at 60FPS
- Chrome: 131000 Bunnies at 60FPS

Let's keep in mind that the conditions for `bunnymark` are very unlikely to see in a real project.
However, it's widely used to test the performance in 2D Draw APIs.

## Integration 

Notan is designed to be as modular as possible. It's flexible enough to allow change how the event life cycle works with 
a plugin (i.e: [FpsLimit](crates/notan_extra/src/fps_limit.rs)), or to allow us to draw custom things easily on top of the 
graphics API using *Graphic Extensions* (i.e: [egui](crates/notan_egui) or [draw](crates/notan_draw)).

Even any backend can be easily *plugged-in* from the code just using `init_with_backend`.

We include some of these plugins or graphics extensions behind feature flags, as a part of the project.
However, everybody can create their own plugins or extension to extend Notan.

## Why?

I have been looking since ever for a project that allows me to create multimedia apps (games in my case) with just one codebase,
not been too much opinionated about how to do it, with multiple platforms support and treating the web as a first-class citizen.

I felt that it was a tricky thing to find until I found [Haxe](https://haxe.org/) and [Kha](https://kha.tech/), the perfect match.
However, I did not like a few things about the build system, the lack of tools and IDEs, and how the language itself does some things.
So, after a while I decided to start looking again, and I saw that **Rust** had a great **WebAssembly** compiler among other targets, 
and check all those boxes.

For the last three years, I have been working on this project in different repositories with different names and multiple "start-over" times.
It was my place to learn Rust and OpenGL, you can say that it was my sandbox and my hobby.

However, I feel that it could be useful for more people than me in the current state.

The name **Notan** comes from `not an engine`. The main purpose of the project is to be used as foundation providing 
a basic but useful set of features. 

They are:
- Platform abstraction (desktop, mobile, etc...)
- Windowing
- Graphics rendering
- Text rendering
- Draw 2D API
- Audio 
- Input (Keyboard, mouse, etc...)
- Simple UI via egui

Everything else, like particles, physics, etc..., is out of the scope of the project
and can be added via external crates, plugins or extensions. Although we can always talk 
if you think that something should be part of the project.

## So... what's next?

Improving the project step by step, aiming for a more mature and better APIs, fixes, better platform support, more graphics backends etc...

## License

This project is licensed under either of [Apache License, Version
2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT), at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache 2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
