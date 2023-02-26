use hashbrown::HashMap;
pub use notan_graphics::prelude::*;
pub use notan_graphics::*;
use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};

/// Graphic interface to interact with the GPU
/// It's a wrapper for the Device interface and
/// the graphics extensions
pub struct Graphics {
    /// Graphic raw implementation
    pub device: Device,

    pub extensions: ExtContainer,
}

impl Graphics {
    pub fn new(backend: Box<dyn DeviceBackend>) -> Result<Self, String> {
        let device = Device::new(backend)?;
        let plugins = ExtContainer::default();

        Ok(Self {
            device,
            extensions: plugins,
        })
    }

    /// Adds a new graphic extensions
    #[inline]
    pub fn add_extension<R, T>(&mut self, extension: T)
    where
        R: GfxRenderer,
        T: GfxExtension<R> + 'static,
    {
        self.extensions.add(extension);
    }

    /// Remove a graphic extensions
    #[inline]
    pub fn remove_extension<R, T>(&mut self)
    where
        R: GfxRenderer,
        T: GfxExtension<R> + 'static,
    {
        self.extensions.remove::<R, T>();
    }

    /// Returns the extension as mutable reference
    #[inline]
    pub fn extension_mut<R, T>(&self) -> Option<RefMut<T>>
    where
        R: GfxRenderer,
        T: GfxExtension<R> + 'static,
    {
        self.extensions.get_mut()
    }

    /// Returns the extension as reference
    #[inline]
    pub fn extension<R, T>(&self) -> Option<Ref<T>>
    where
        R: GfxRenderer,
        T: GfxExtension<R> + 'static,
    {
        self.extensions.get()
    }

    /// Creates a Pipeline builder
    #[inline]
    pub fn create_pipeline(&mut self) -> PipelineBuilder {
        self.device.create_pipeline()
    }

    /// Creates a texture builder
    #[inline]
    pub fn create_texture(&mut self) -> TextureBuilder {
        self.device.create_texture()
    }

    /// Creates a render texture builder
    #[inline]
    pub fn create_render_texture(&mut self, width: u32, height: u32) -> RenderTextureBuilder {
        self.device.create_render_texture(width, height)
    }

    /// Creates a vertex buffer builder
    #[inline]
    pub fn create_vertex_buffer(&mut self) -> VertexBufferBuilder {
        self.device.create_vertex_buffer()
    }

    /// Creates a index buffer builder
    #[inline]
    pub fn create_index_buffer(&mut self) -> IndexBufferBuilder {
        self.device.create_index_buffer()
    }

    /// Creates a uniform buffer builder
    #[inline]
    pub fn create_uniform_buffer(&mut self, slot: u32, name: &str) -> UniformBufferBuilder {
        self.device.create_uniform_buffer(slot, name)
    }

    /// Update the texture data
    #[inline]
    pub fn update_texture<'a>(&'a mut self, texture: &'a mut Texture) -> TextureUpdater {
        self.device.update_texture(texture)
    }

    /// Read pixels from a texture
    #[inline]
    pub fn read_pixels<'a>(&'a mut self, texture: &'a Texture) -> TextureReader {
        self.device.read_pixels(texture)
    }

    /// Render to the screen
    #[inline]
    pub fn render<G: GfxRenderer>(&mut self, renderer: &G) {
        if let Err(err) = renderer.render(&mut self.device, &mut self.extensions, None) {
            log::error!("{}", err);
            panic!("{}", err);
        }
    }

    /// Render to a custom target
    #[inline]
    pub fn render_to<G: GfxRenderer>(&mut self, target: &RenderTexture, renderer: &G) {
        if let Err(err) = renderer.render(&mut self.device, &mut self.extensions, Some(target)) {
            log::error!("{}", err);
            panic!("{}", err);
        }
    }

    /// Upload the buffer data to the GPU
    #[inline]
    pub fn set_buffer_data<T: BufferData>(&mut self, buffer: &Buffer, data: T) {
        self.device.set_buffer_data(buffer, data);
    }

    /// Creates a render pass
    #[inline]
    pub fn create_renderer(&self) -> Renderer {
        self.device.create_renderer()
    }

    /// Returns the Graphics API limits
    #[inline]
    pub fn limits(&self) -> Limits {
        self.device.limits()
    }

    /// Returns the drawable size
    #[inline]
    pub fn size(&self) -> (u32, u32) {
        self.device.size()
    }

    /// Sets the drawable size
    #[inline]
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.device.set_size(width, height);
    }

    /// Returns the screen dpi
    #[inline]
    pub fn dpi(&self) -> f64 {
        self.device.dpi()
    }

    /// Sets the screens dpi
    #[inline]
    pub fn set_dpi(&mut self, scale_factor: f64) {
        self.device.set_dpi(scale_factor);
    }

    /// Return the GPU stats
    #[inline]
    pub fn stats(&self) -> GpuStats {
        self.device.stats()
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
#[derive(Default)]
pub struct ExtContainer {
    map: HashMap<TypeId, Box<dyn Any>>,
}

impl ExtContainer {
    /// Adds a graphics extension
    #[inline]
    pub fn add<R, T>(&mut self, value: T)
    where
        R: GfxRenderer,
        T: GfxExtension<R> + 'static,
    {
        self.map
            .insert(TypeId::of::<T>(), Box::new(RefCell::new(value)));
    }

    /// Returns the extension as mutable reference
    #[inline]
    pub fn get_mut<R, T>(&self) -> Option<RefMut<'_, T>>
    where
        R: GfxRenderer,
        T: GfxExtension<R> + 'static,
    {
        self.map
            .get(&TypeId::of::<T>())?
            .downcast_ref::<RefCell<T>>()
            .map(|value| value.borrow_mut())
    }

    /// Returns the extension
    #[inline]
    pub fn get<R, T>(&self) -> Option<Ref<'_, T>>
    where
        R: GfxRenderer,
        T: GfxExtension<R> + 'static,
    {
        self.map
            .get(&TypeId::of::<T>())?
            .downcast_ref::<RefCell<T>>()
            .map(|value| value.borrow())
    }

    /// Remove the extension
    #[inline]
    pub fn remove<R, T>(&mut self)
    where
        R: GfxRenderer,
        T: GfxExtension<R> + 'static,
    {
        self.map.remove(&TypeId::of::<T>());
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
    ) -> Result<(), String>;
}

pub trait GfxExtension<T: ?Sized> {}

impl GfxRenderer for Renderer {
    fn render(
        &self,
        device: &mut Device,
        _extensions: &mut ExtContainer,
        target: Option<&RenderTexture>,
    ) -> Result<(), String> {
        match target {
            None => device.render(self.commands()),
            Some(rt) => device.render_to(rt, self.commands()),
        }

        Ok(())
    }
}
