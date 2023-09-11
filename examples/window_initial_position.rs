use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    let win = WindowConfig::default().set_position(100, 100);
    notan::init().add_config(win).build()
}
