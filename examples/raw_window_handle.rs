use notan::draw::*;
use notan::prelude::*;
use raw_window_handle::{AppKitWindowHandle, RawWindowHandle};

#[notan_main]
fn main() -> Result<(), String> {
    let win = WindowConfig::default()
        .set_transparent(true)
        .set_decorations(false);
    notan::init_with(setup)
        .add_config(win)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .draw(draw)
        .build()
}

fn setup(app: &mut App) {
    let raw_window_handle = app.window().raw_window_handle().unwrap();

    match raw_window_handle {
        RawWindowHandle::AppKit(AppKitWindowHandle { ns_view, .. }) => {
            #[cfg(target_os = "macos")]
            unsafe {
                use cocoa::appkit::NSWindow;
                use cocoa::base::NO;
                use objc::runtime::Object;
                use objc::{msg_send, sel, sel_impl};

                // I told you it was unsafe.
                let ns_view: *mut Object = ns_view as *mut Object;
                let ns_window: *mut Object = msg_send![ns_view, window];

                // NOTE: remove shadow to stop artifacts in transparent window
                ns_window.setHasShadow_(NO);
                ns_window.setLevel_(1001);
                println!("{:?}", raw_window_handle)
            }
        }
        _ => {
            println!("{:?}", raw_window_handle)
        }
    }
}

fn draw(gfx: &mut Graphics) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::TRANSPARENT);
    draw.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0))
        .color(Color::MAGENTA);
    gfx.render(&draw);
}
