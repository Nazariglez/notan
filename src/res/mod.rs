mod blob;
mod manager;

pub use backend::{BaseApp, Font, System, Texture};

use crate::app::App;
pub use blob::*;
pub(crate) use manager::*;

pub trait ResourceParser {
    type App: BaseApp;

    fn parse_res(&mut self, app: &mut Self::App, data: Vec<u8>) -> Result<(), String>;
    fn already_loaded(&mut self) -> bool;
}

#[macro_export]
macro_rules! resource_parser {
    ($type:ty, $app:ty) => {
        impl ResourceParser for $type {
            type App = $app;

            fn parse_res(&mut self, app: &mut $app, data: Vec<u8>) -> Result<(), String> {
                self.parse_data(app, data)
            }

            fn already_loaded(&mut self) -> bool {
                self.is_loaded()
            }
        }
    };
}

resource_parser!(backend::Texture, App);
resource_parser!(backend::Font, App);
