use super::manager::AssetManager;
use super::storage::AssetStorage;
use crate::app::App;
use downcast_rs::{impl_downcast, Downcast};
use std::any::TypeId;
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct Loader {
    extensions: Vec<String>,
    parser: Option<LoaderCallback>,
    type_id: Option<TypeId>,
}

impl Loader {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn output<A>(mut self) -> Self
    where
        A: Send + Sync + 'static,
    {
        self.type_id = Some(TypeId::of::<A>());
        self
    }

    pub fn from_extension(mut self, ext: &str) -> Self {
        self.extensions.push(ext.to_string());
        self
    }

    pub fn from_extensions(mut self, exts: &[&str]) -> Self {
        for ext in exts {
            self.extensions.push(ext.to_string());
        }
        self
    }

    pub fn use_parser<H, Params>(mut self, handler: H) -> Self
    where
        H: LoaderHandler<Params>,
    {
        self.parser = Some(handler.callback());
        self
    }

    pub(crate) fn apply(self, manager: &mut AssetManager) -> Result<(), String> {
        let Loader {
            extensions,
            parser,
            type_id,
        } = self;

        if extensions.is_empty() {
            return Err("Loader without extensions associated.".to_string());
        }

        let type_id =
            type_id.ok_or_else(|| "Loader without output type associated.".to_string())?;
        let mut parser = parser.ok_or_else(|| "Loader without parser associated.".to_string())?;
        parser.set_type_id(type_id);

        extensions.iter().for_each(|ext| {
            manager.loaders.insert(ext.to_string(), parser.clone());
        });

        Ok(())
    }
}

#[derive(Clone)]
pub enum LoaderCallback {
    Basic(
        Option<TypeId>,
        Rc<dyn Fn(&str, Vec<u8>, &mut AssetStorage) -> Result<(), String>>,
    ),
}

pub trait LoaderHandler<Params> {
    fn callback(self) -> LoaderCallback;
}

macro_rules! loader_handler {
    ($variant:expr, $($param:ident),*) => {
        #[allow(unused_parens)]
        impl<F> LoaderHandler<(&str, Vec<u8>, &mut AssetStorage, $(&mut $param),*)> for F
        where
            F: Fn(&str, Vec<u8>, &mut AssetStorage, $(&mut $param),*) -> Result<(), String> + 'static
        {
            fn callback(self) -> LoaderCallback {
                $variant(None, Rc::new(self))
            }
        }
    }
}

loader_handler!(LoaderCallback::Basic,);

impl LoaderCallback {
    pub(crate) fn exec(
        &self,
        id: &str,
        data: Vec<u8>,
        storage: &mut AssetStorage,
    ) -> Result<(), String> {
        use LoaderCallback::*;
        match self {
            Basic(_, cb) => cb(id, data, storage),
        }
    }

    pub(crate) fn set_type_id(&mut self, type_id: TypeId) {
        use LoaderCallback::*;
        let ty = match self {
            Basic(ref mut ty, _) => ty,
        };

        *ty = Some(type_id);
    }

    pub(crate) fn type_id(&self) -> Option<TypeId> {
        use LoaderCallback::*;
        match self {
            Basic(ty, _) => *ty,
        }
    }
}
