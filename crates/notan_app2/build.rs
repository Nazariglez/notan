use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        is_web: { target_arch = "wasm32" },
        is_winit: { all(not(is_web), feature = "winit") },
        is_empty: { all(not(is_web), not(is_winit)) }
    }
}
