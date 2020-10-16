mod blob;
mod manager;

pub use backend::{BaseApp, Font, Resource, System, Texture};

use crate::app::App;
pub use blob::*;
pub(crate) use manager::*;

pub trait ResourceParser {
    type App: BaseApp;

    fn parse_resource(&mut self, app: &mut Self::App, data: Vec<u8>) -> Result<(), String>;
}

#[macro_export]
macro_rules! resource_parser {
    ($type:ty, $app:ty) => {
        impl ResourceParser for $type {
            type App = $app;

            fn parse_resource(&mut self, app: &mut $app, data: Vec<u8>) -> Result<(), String> {
                self.set_data(app, data)
            }
        }
    };
}

resource_parser!(backend::Texture, App);
resource_parser!(backend::Font, App);
