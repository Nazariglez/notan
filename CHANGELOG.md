# Changelog
All notable changes to this project will be documented in this file.

## v0.13.0 - UNRELEASED

## v0.12.1 - 08/06/2024

- Updated EGUI to `0.27`.
- The readme has gifs again.
- Added `app.mouse.clean_button_state` to clean the state of a button.
- Added `xtask` to run project's script like building the examples for web.
- Fixed an issue with EGUI that makes text looks blurry.

## v0.12.0 - 19/02/2024

- Updated EGUI to `0.26`.
- Removed `egui::plugin::Output.needs_repaint()`, now is only used internally and not exposed to users.
- Exposed `notan::draw::DrawBuilder` allowing custom builders.
- Exposed `notan::app::AppTimer`.
- Added `draw.point` allowing to draw points. Check `examples/draw_point.rs`.
- Allow to compile the crate without a backend selected.
- Changed `WindowConfig::set_canvas_id` to `WindowConfig::set_app_id` and is not available for wayland too.
- Fixed `app.request_frame()` when using lazy lopps on Window OS.

## v0.11.0 - 18/10/2023

- Added traits `Serialize` and `Deserialize` to `Color` with the feature `serde` enabled.
- Updated EGUI to `0.23`.
- Fixed an error acquiring the GL Context due required samples configuration.

## v0.10.0 - 11/09/2023

- Added `WindowConfig::set_position` to set x/y position before creating the window.
- Changed `Renderer.begin` uses `Option<ClearOption>` instead of `Option<&ClearOption>`.
- Changed sizes and positions for Window and Textures from `i32` to `u32`.
- Added `AppTimer::elapsed` to return time since init as `Duration`.
- Changed `AppTimer::time_since_init` to `AppTimer::elapsed_f32`.
- Changed `WindowConfig` setter method to use the prefix `set_`. 
- Removed deprecated `Mouse::local_position`.
- Removed deprecated `mat3_screen_to_local`, `mat3_local_to_screen`, `mat3_local_to_local`.
- Updated dependencies to latest versions. 
- Enabled compilation with `--no-default-features` excluding shader compilation macros.
- Deserializing `AtlasFrame` uses a default `pivot` if is empty.
- Added `WindowConfig::set_window_icon_data`.
- Added `WindowConfig::set_taskbar_icon_data`.
- Added example `window_icon_from_raw.rs`.
- Changed `glsl_layout` dependency for `crevice`.
- Updated EGUI to `0.22`.
- Fixed `egui` panic when custom font are set.  
- Fixed slow scroll speed. 
- Fixed `egui needs_repaint` not working right in some situations.
- Fixed the order of the matrix multiplication for `Draw` methods.
- Improved error messages when `WebGL` and `WebGL2` contexts cannot be adquired.
- Fixed `Buffer` to allow reuse `Uniform Buffers` between pipelines.
- Changed some noisy logs from `debug` to `trace`.
- Added `Clone` to `Random`.
- Reset values of `Mouse::wheel_delta` when the user stops scrolling.
- Added `Mouse::is_scrolling`.
- App's state can use now lifetimes, ie: `State<'n>`.
- Added `Clone` to `AssetsList`.
- The `image` crate on `notan_graphics` is only used when `texture_to_file` is enabled.
- Added `WindowBackend::set_cursor_position`, `Event::MouseMotion` and `Mouse::is_moving`.
- Added new example `window_initial_position.rs`.
- Added mipmap and texture wrapping settings to `RenderTextureBuilder`.
- Added new example `texture_params`.
- Added new example `renderer_stencil`.
- Fixed mouse wheel scroll being ignored when moving the mouse at same time
- Added alt mouse wheel scrolling code to example
- Fixed `set_multisamples`. It is no longer being ignored for winit backend
- Fixed blurry text on egui when using on desktop
- Fixed mono channel audio playing in half of time set for the audio length. 
- Added `is_focused()` for winit backend
- Added `window_focus` example

## v0.9.5 - 19/03/2023

- Increased mouse wheel scroll speed on native platforms.
- Added `WindowBackend::set_touch_as_mouse` and `touch_as_mouse` to enable/disable it at runtime.
- Fix `Event::Exit` which is triggered now before the app is closed.
- Add `WindowConfig::set_window_icon_data` and `set_taskbar_icon_data` to set them using bytes. Check `examples/window_icon_from_raw.rs` for more info.
- Allow to load images without allocation limits, return an error if the image is bigger than the size supported by the GPU. 

## v0.9.4 - 26/02/2023

- Added `WindowBackend::set_title` and `WindowBackend::title` to change or get the title at any time.
- Improved stencil clearing when setting a mask on the draw api at the end of the pass.
- Fix panic using Draw API to draw text with `max_width` and `size` as 0. 
- Fix `debug_assert` in `Device::inner_read_pixels`.
- Added support for `include` directives using `shaderc`.
- Added optional feature `serde` to serialize/deserialize some core types.

## v0.9.3 - 12/02/2023

- Added `WindowBackend::screen_size` to get the screen's resolution size.
- Added `WindowBackend::container_size` to get the windows container size (screen on native, parent element on web).
- Fix Draw2D masking issue about the stencil clearing.
- New example `draw_mask_animated.rs`.

## v0.9.2 - 05/02/2023

- Added `Fn` keys to `egui`.
- New example `game_tic_tac_toe.rs`.
- Fix corners of shapes using the Draw2D API. 
- New example `draw_text_max_width.rs`.
- Fix window's transparency issue on x11 linux.
- Fix an issue with some windows to select the OpenGL context. 

## v0.9.1 - 26/01/2023

- Fix docs compilation.

## v0.9.0 - 24/01/2023

- Fix alpha blending mode issue with text rendering using the Draw2D API.
- Improve how the alpha blending behaves rendering from and to `RenderTexture` using Draw2D API.
- Fix `Draw` structure is clonable again.
- Change `SetupHandler` and `AppBuilder::initialize` to `FnOnce` allowing to embed notan easily.
- Updated the crate `glutin` to `0.30.2`. 
- New example `draw_arcs.rs` to show how to draw circle sections.
- Added new texture format `R8Uint`.
- Draw unsupported chars with a font does not panic anymore. 
- Added `WindowConfig::window_icon` and `WindowConfig::taskbar_icon` to add icons for windows os.
- Added example `egui_custom_font.rs`.
- Fix images loaded from files can set the texture format other rgba.
- Added `TextureFormat::Rgba32Float`.
- Avoid some allocations when textures are loaded.

## v0.8.0 - 28/11/2022

- Updated `notan_egui` to the latest version of `egui` 0.19.
- Added mipmapping support with `TextureBuilder::generate_mipmaps`.
- Added `WindowBackend::position` and `WindowBackend::set_position`.
- Fix lint warning `notan_main` macro.
- Added methods `.fill_color` and `stroke_color` for the Draw2d shapes to allow to stroke and fill with the same builder.
- Added method `Draw::star(spikes, outser_radius, inner_radius)` to draw stars.
- Added method `Draw::polygon(sides, radius)` to draw regular polygons.
- Added `shaderc` feature to compile shaders using `shaderc` instead of `glsl_to_spirv`.
- Fix `RenderTexture` orientation when drawing using the Draw2d API.
- Added `IndexBufferBuilder::with_data_u16` to create index buffers using u16 slices.
- Added `Text::last_bounds` to get the bounding box of the latest text drawn.
- Added `Text::bounds` to get the bounding box of all the text elements combined.
- Added `Draw::last_text_bounds` to get the bounding box of the latest text drawn using the Draw2d API.
- New examples `text_bounds.rs` and `draw_text_bounds.rs` to show how to measure the text size with real use cases.
- Added a CI action to check if the code meets a minimal quality conditions.
- Added `WindowBackend::set_mouse_passthrough` to change the passthrough condition at runtime.
- Fix custom pipelines for the Draw2d APIs. They were working only for images, now they work all (shapes, patterns, etc..)
- Added example `draw_shapes_shader.rs` to show how to set a custom pipeline drawing shapes.
- Renamed `draw_shader.rs` to `draw_image_shader.rs`
- Added `Graphics::stats() -> GpuStats` to get more info about what the GPU did the last frame.
- Added new texture formats. `TextureFormat::R16Uint`, `R32Uint`, `R32Float`.
- New example `renderer_texture_r32.rs` to show how to use new texture types.
- The method `Renderer::bind_texture` will set the slot automatically to the next one if using in a row.
- Replaced `copypasta` dependency by `arboard` and moved clipboard features to app level.
- Added clipboard support for web browsers using `wasm`.
- Added `.flip_x` and `.flip_y` to `Image`, `Animation` and `Pattern` object from the Draw2d API.
- Changed `Draw::set_blend_mode` needs an `Option<BlendMode>` now, and passing None the blending mode can be canceled.
- Added `Draw::set_alpha_mode` and `DrawBuilder::alpha_mode` to set the blend mode for the alpha composition.

## v0.7.1 - 08/10/2022

- Added support for clipboard events using `egui` behind the feature `clipboard`.
- Exposed `GlowBackend::add_inner_texture` to allow more flexibility extending the backend.
- Example `input_keyboard` uses not `delta time`.
- Added `WindowConfig::mouse_passtrhough` to allow mouse events to pass through the window.
- Fix a minor bug in the `egui` plugin recognizing the `CMD` key on `osx`.

## v0.7.0 - 29/09/2022

- Updated and upgraded all dependencies
- Fix audio bug that starts a sound with maximum volume and then fade.
- Added `WindowConfig::always_on_top` and `WindowBackend::set_always_on_top/is_always_on_top` to force the window to the foreground. Has no effect on the web.
- Added `notan_random` and feature `random` to allow users to disable the default random features and use their own.
- In EguiPlugin, handle `CMD` key on web.
- Fix, inverted the direction of the horizontal mouse wheel on web.
- Added `TextureBuilder::from_source(raw)` to create textures that are backend dependent.
- Added `TextureUpdater::with_source(raw)` to update textures that are backend dependent.
- Added support to load and update `web_sys::HtmlImageElement` using the default backend.

## v0.6.0 - 27/08/2022

- Fix the syntax in some example's shader.
- Glam type can be used as uniforms directly.
- Add `#[uniform]` macro to layout the data as `std140`.
- On MacOS, disabled the high dpi resolution by default.
- On Web, disabled the high dpi resolution by default.
- Added `WindowConfig::high_dpi` to enable high resolution on MacOS and Web.
- Added `Draw::screen_to_world_position` and `Draw::world_to_screen_position` to convert coordinates.
- Added `DrawBuilder::screen_to_local_position` and `DrawBuilder::local_to_screen_position` to convert coordinates.
- Fix 15 Puzzle game bug.
- Change `WindowConfig` to take values instead of set the `!default` value.
- Fix `wasm32` warnings due a leaked reference.
- Add `WindowConfig::canvas_id` to use or create a custom canvas.
- Remove the deprecated `notan::math::DEG_TO_RAD` and `notan::math::RAD_TO_DEG`.
- Fix using `lazy_mode` an empty buffer after the first swap buffers.
- Add `draw_projection.rs` example.
- Add `extra` feature and `notan_extra` crate to add utils/struct that doesn't fit in other crates.
- Add `extra::FpsLimit` to limit the maximum framerate and save CPU cycles putting it to sleep.
- Removed `app::FpsPlugin` in favour of `extra::FpsLimit`.

## v0.5.1  04/07/2022

- Fixed window shader compilation.
- Egui will call RequestRedraw when there is some animation, no need to call it manually anymore.

## v0.5.0 - 26/06/2022

- Removed chrono due to a security issue.
- Fixed viewport issues where the Y axis was inverted and wasn't using DPI to calculate min position.
- Fixed EGUI 0.18.1. Paint callback feature.
- Added `Window::set_capture_cursor` and `Window::capture_cursor` to confine the cursor into the window's app.
- Added `app.mouse.wheel_delta` to read the delta without checking the event loop.
- Added `texture_to_file` feature to save textures as png files. Use `Texture::to_file(gfx, path)` and `RenderTexture::to_file(gfx, path)`.
- Window can be hidden or displayer now setting the visibility on `WindowConfig` and `Window::set_visible`.
- Fixed `wasm32` mouse drag event (it had a conflict with pointerevents).
- Inlined docs for re-exported crates.
- Added `debug` checks for some OpenGL actions to avoid panics without info for bad API use.
- Added example for `texture_to_file`.

## v0.4.2 - 16/06/2022

* VAOs doesn't keep older attribute pointers anymore after a new VAO is bind
* Textures can use Wrap modes now with `TextureBuilder .with_wrap(x, y) // s,t`
* Increased textures slots per shader from 8 to 16
* Deprecated `math::DEG_TO_RAD` and `math::RAD_TO_DEG` (rust provides `.to_radians()` and `.to_degrees()`)
* Textures need to be declared on the pipeline with the location ID `.with_texture_location(0, "uniform_id")`
* Added `15puzzle` example

## v0.4.1 - 04/06/2022

- Added transparent and decorations windows options
- Removed `winit` (`glutin` already used it)
- Fix rotation issues with draw text

## v0.4.0 - 15/05/2022

* Added touch support
* Audio API requires an initial volume when the sound is created
* Updated `egui` to 0.18.1
* Fix runtime error using Wayland and Mesa
* Dependencies updated to the latest version
* Created a new crate `notan_input` to keep organized all the user's input code

## v0.3.0 - 29/04/2022

- Updated dependencies.
- Added mouse and keyboard types to the prelude.
- Audio Support using `oddio`.

## v0.2.1 - 29/03/2022

- Force `egui` to repaint after a window's resize.

## v0.2.0 - 27/03/2022

- Updated all dependencies to the latest version available.
- Migrated from 2018 edition to 2021.
- Added drag and drop file support.
- Added custom mouse cursors support.
- Added browser links support.
- Added lazy loop mode.
- Added `sRGB` texture support.
- Added `Lines` and `LineStrip` primitives.
- Added new examples and improved the main page.
- Added support for premultiplied alpha images.
- Re-exported `glam` types as part of `notan_math`.
- Renamed `VertexFormat` to more explicit names.
- Improved the API used to call `egui` code.
- Other minor fixes and changes...

## v0.1.0 - 20/02/2022

- Initial version.
