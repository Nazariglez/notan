mod blob;
mod manager;

pub use backend::{Font, Resource, System, Texture};

pub use blob::*;
pub(crate) use manager::*;
use nae_core::BaseSystem;

pub trait ResourceParser {
    type System: BaseSystem;

    fn parse_res(&mut self, app: &mut Self::System, data: Vec<u8>) -> Result<(), String>;
    fn already_loaded(&mut self) -> bool;
}

#[macro_export]
#[macro_use]
macro_rules! resource_parser {
    ($type:ty, $system:ty) => {
        impl ResourceParser for $type {
            type System = $system;

            fn parse_res(&mut self, sys: &mut $system, data: Vec<u8>) -> Result<(), String> {
                self.parse(sys, data)
            }

            fn already_loaded(&mut self) -> bool {
                self.is_loaded()
            }
        }
    };
}

resource_parser!(backend::Texture, System);
resource_parser!(backend::Font, System);
