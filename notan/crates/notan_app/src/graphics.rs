use hecs::{Entity, Ref, World};
pub use notan_graphics::prelude::*;
pub use notan_graphics::*;

/// Graphic interface to interact with the GPU
pub struct Graphics {
    /// Graphic raw implementation
    pub device: Device,

    /// Graphics extensions
    pub extensions: ExtContainer,
}

impl Graphics {
    pub fn new(backend: Box<DeviceBackend>) -> Result<Self, String> {
        let mut device = Device::new(backend)?;
        let plugins = ExtContainer::default();

        Ok(Self {
            device,
            extensions: plugins,
        })
    }

    /// Adds a new graphic extensions
    #[inline]
    pub fn add_ext<R, T>(&mut self, extension: T)
    where
        R: GfxRenderer,
        T: GfxExtension<R> + 'static,
    {
        self.extensions.add(extension);
    }

    /// Remove a graphic extensions
    #[inline]
    pub fn remove_ext<R, T>(&mut self)
    where
        R: GfxRenderer,
        T: GfxExtension<R> + 'static,
    {
        self.extensions.remove::<R, T>();
    }

    /// Creates a Pipeline builder
    #[inline]
    pub fn create_pipeline(&mut self) -> PipelineBuilder {
        PipelineBuilder::new(&mut self.device)
    }

    /// Creates a texture builder
    #[inline]
    pub fn create_texture(&mut self) -> TextureBuilder {
        TextureBuilder::new(&mut self.device)
    }

    /// Creates a render texture builder
    #[inline]
    pub fn create_render_texture(&mut self, width: i32, height: i32) -> RenderTextureBuilder {
        RenderTextureBuilder::new(&mut self.device, width, height)
    }

    /// Creates a vertex buffer builder
    #[inline]
    pub fn create_vertex_buffer(&mut self) -> BufferBuilder<f32> {
        BufferBuilder::new(&mut self.device, BufferUsage::Vertex, None)
    }

    /// Creates a index buffer builder
    #[inline]
    pub fn create_index_buffer(&mut self) -> BufferBuilder<u32> {
        BufferBuilder::new(&mut self.device, BufferUsage::Index, None)
    }

    /// Creates a uniform buffer builder
    #[inline]
    pub fn create_uniform_buffer(&mut self, slot: u32, name: &str) -> BufferBuilder<f32> {
        BufferBuilder::new(&mut self.device, BufferUsage::Uniform(slot), Some(name))
    }

    /// Render to the screen
    #[inline]
    pub fn render(&mut self, renderer: &GfxRenderer) {
        renderer.render(&mut self.device, &mut self.extensions, None);
    }

    /// Render to a custom target
    #[inline]
    pub fn render_to(&mut self, target: &RenderTexture, renderer: &GfxRenderer) {
        renderer.render(&mut self.device, &mut self.extensions, Some(target));
    }
}

impl std::ops::Deref for Graphics {
    type Target = Device;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

impl std::ops::DerefMut for Graphics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.device
    }
}

/// Graphic extensions container
pub struct ExtContainer {
    world: World,
    entity: Entity,
}

impl Default for ExtContainer {
    fn default() -> Self {
        let mut world = World::new();
        let entity = world.reserve_entity();
        Self { world, entity }
    }
}

impl ExtContainer {
    /// Adds a graphics extension
    pub fn add<R, T>(&mut self, value: T)
    where
        R: GfxRenderer,
        T: GfxExtension<R> + 'static,
    {
        self.world.insert_one(self.entity, value);
    }

    /// Returns the extension as mutable reference
    pub fn get_mut<R, T>(&self) -> Option<hecs::RefMut<'_, T>>
    where
        R: GfxRenderer,
        T: GfxExtension<R> + 'static,
    {
        self.world.get_mut(self.entity).ok()
    }

    /// Returns the extension
    pub fn get<R, T>(&self) -> Option<Ref<'_, T>>
    where
        R: GfxRenderer,
        T: GfxExtension<R> + 'static,
    {
        self.world.get(self.entity).ok()
    }

    /// Remove the extension
    pub fn remove<R, T>(&mut self)
    where
        R: GfxRenderer,
        T: GfxExtension<R> + 'static,
    {
        self.world.remove_one::<T>(self.entity);
    }
}

/// Represents an object that contains render commands
pub trait GfxRenderer {
    /// Send the commands to the gpu to be rendered
    fn render(
        &self,
        device: &mut Device,
        extensions: &mut ExtContainer,
        target: Option<&RenderTexture>,
    );
}

pub trait GfxExtension<T: ?Sized>
where
    Self: Send + Sync,
{
    /// Process and returns the commands
    fn commands<'a>(&'a mut self, device: &mut Device, renderer: &'a T) -> &'a [Commands];
}

impl GfxRenderer for Renderer {
    fn render(
        &self,
        device: &mut Device,
        _extensions: &mut ExtContainer,
        target: Option<&RenderTexture>,
    ) {
        match target {
            None => device.render(self.commands()),
            Some(rt) => device.render_to(rt, self.commands()),
        }
    }
}
