# Changelog
All notable changes to this project will be documented in this file.

## Unreleased

- Fix the syntax in some example's shader.
- Glam type can be used as uniforms directly
- Add `#[uniform]` macro to layout the data as `std140`
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


