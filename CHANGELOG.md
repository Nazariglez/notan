# Changelog
All notable changes to this project will be documented in this file.

## UNRELEASED

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
- Added `TextureBuilder::from_source(raw)` to create textures that are backend dependant. 
- Added `TextureUpdater::with_source(raw)` to update textures that are backend dependant.
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
- Add `WindoConfig::canvas_id` to use or create a custom canvas.
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
- Fixed viewport issues where the Y axis was inverted and wasn't using DPI to calculate min positon.
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
