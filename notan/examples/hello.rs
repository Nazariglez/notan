use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::graphics::prelude::*;
use notan::app::{App, AppBuilder, Plugins};
use notan::log;
use notan::prelude::*;

// #[derive(Default)]
struct State {
    // count: u32,
    // hello: Asset<Vec<u8>>,
    // list: AssetList,
    clear_options: ClearOptions,
    pipeline: Pipeline,
    vertices: [f32; 21],
    vertex_buffer: Buffer,
}

impl AppState for State {}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .set_config(WindowConfig::new().size(1200, 800))
        // .update(update)
        // .draw(draw)
        .build();

    Ok(())
}

fn setup(gfx: &mut Graphics) -> State {
    let clear_options = ClearOptions::new(Color::new(0.1, 0.2, 0.3, 1.0));

    let pipeline = gfx
        .create_pipeline(
            include_bytes!("hello.rs"),
            include_bytes!("hello.rs"),
            &[
                VertexAttr::new(0, VertexFormat::Float3),
                VertexAttr::new(1, VertexFormat::Float4),
            ],
            PipelineOptions::default(),
        )
        .unwrap();

    #[rustfmt::skip]
    let vertices = [
        -0.5, -0.5, 0.0,    1.0, 0.2, 0.3, 1.0,
        0.5, -0.5, 0.0,     0.1, 1.0, 0.3, 1.0,
        0.0, 0.5, 0.0,      0.1, 0.2, 1.0, 1.0,
    ];

    let vertex_buffer = gfx.create_vertex_buffer(DrawType::Static).unwrap();

    let mut state = State {
        clear_options,
        pipeline,
        vertices,
        vertex_buffer,
    };

    draw(&mut state, gfx);

    state
}

fn draw(state: &mut State, gfx: &mut Graphics) {
    let mut renderer = gfx.create_renderer();

    renderer.begin(&ClearOptions::new(Color::new(0.1, 0.2, 0.3, 1.0)));
    renderer.set_pipeline(&state.pipeline);
    renderer.bind_vertex_buffer(&state.vertex_buffer, &state.vertices);
    renderer.draw(0, 3);
    renderer.end();

    gfx.render(&renderer);
}
//
// fn setup(assets: &mut AssetManager) -> State {
//     State {
//         // count: 0,
//         // hello: assets.load_asset::<Vec<u8>>("hello.html").unwrap(),
//         // list: assets.load_list(&["hello.html", "hell.html"]).unwrap(),
//     }
// }
//
// fn update(app: &mut App, state: &mut State) {
//     // log::info!(
//     //     "asset: {:?} -> list: {:?} - {:?}",
//     //     state.hello.is_loaded(),
//     //     state.list.is_loaded(),
//     //     state.list.progress()
//     // );
//     //
//     // state.count += 1;
// }
//
// #[notan::shader]
// struct PaintShader {
//     a: i32,
//     b: i32,
//     c: i32,
// }
