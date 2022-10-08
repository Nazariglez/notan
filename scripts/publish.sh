# Script taken from BevyEngine https://github.com/bevyengine/bevy/blob/main/tools/publish.sh

# if crate A depends on crate B, B must come before A in this list
crates=(
  notan_core
  notan_input
  notan_audio
  notan_random
  notan_utils
  notan_math
  notan_macro
  notan_graphics
  notan_app
  notan_log
  notan_glow
  notan_oddio
  notan_glyph
  notan_egui
  notan_text
  notan_draw
  notan_web
  notan_winit
  notan_backend
  notan_extra
)

cd crates
for crate in "${crates[@]}"
do
  echo "Publishing ${crate}"
  (cd "$crate"; cargo publish --no-verify)
  sleep 30
done

cd ..
cargo publish
