use hashbrown::HashMap;
use super::resource::*;
use super::loader::Loader;

pub struct ResourceManager<'a> {
    to_load: HashMap<String, Box<Resource+'a>>,
}

impl<'a> ResourceManager<'a> {
    pub fn new() -> Self {
        Self {
            to_load: HashMap::new()
        }
    }

    //TODO improve the loader, right now is useless
    pub fn loader(&'a mut self) -> Loader {
        Loader::new(self)
    }

    pub fn load<R>(&mut self, file:&str) -> Result<R, String>
        where R: Resource + ResourceConstructor + Clone + 'a
    {
        let asset = R::new(file);
        self.to_load.insert(file.to_string(), Box::new(asset.clone()));
        Ok(asset)
    }

    pub fn try_load(&mut self) -> Result<(), String> {
        if self.to_load.len() == 0 {
            return Ok(());
        }

        let mut loaded_files = vec![];
        for (f, a) in self.to_load.iter_mut() {
            a.try_load()?;
            if a.is_loaded() {
                loaded_files.push(f.clone());
            }
        }

        for f in loaded_files {
            self.to_load.remove(&f);
        }

        Ok(())
    }

    pub fn clear(&mut self) {
        self.to_load.clear();
    }
}

