use notan::app::App;

fn main() -> Result<(), String> {
    notan::init().add_config(App::config())?.build()
}
