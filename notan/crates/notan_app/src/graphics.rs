pub use notan_graphics::prelude::*;
pub use notan_graphics::*;

pub struct Graphics {
    pub device: Device,
    // draw: DrawManager,
    //
    // #[cfg(feature = "glyphs")]
    // glyphs: GlyphManager,
    pub plugins: GfxPlugins,
}

impl Graphics {
    pub fn new(backend: Box<DeviceBackend>) -> Result<Self, String> {
        let mut device = Device::new(backend)?;
        // let draw = DrawManager::new(&mut device)?;

        // #[cfg(feature = "glyphs")]
        // let glyphs = GlyphManager::new(&mut device)?;

        let mut plugins = GfxPlugins::default();
        // plugins.set(RenderPlugin);
        // plugins.set(Draw2DPlugin {
        //     manager: DrawManager::new(&mut device)?,
        //     glyphs: GlyphManager::new(&mut device)?
        // });

        Ok(Self {
            device,

            //
            // #[cfg(feature = "glyphs")]
            // glyphs,
            plugins,
        })
    }

    pub fn create_pipeline(&mut self) -> PipelineBuilder {
        PipelineBuilder::new(&mut self.device)
    }

    pub fn create_texture(&mut self) -> TextureBuilder {
        TextureBuilder::new(&mut self.device)
    }

    pub fn create_render_texture(&mut self, width: i32, height: i32) -> RenderTextureBuilder {
        RenderTextureBuilder::new(&mut self.device, width, height)
    }

    pub fn create_vertex_buffer(&mut self) -> BufferBuilder<f32> {
        BufferBuilder::new(&mut self.device, BufferUsage::Vertex, None)
    }

    pub fn create_index_buffer(&mut self) -> BufferBuilder<u32> {
        BufferBuilder::new(&mut self.device, BufferUsage::Index, None)
    }

    pub fn create_uniform_buffer(&mut self, slot: u32, name: &str) -> BufferBuilder<f32> {
        BufferBuilder::new(&mut self.device, BufferUsage::Uniform(slot), Some(name))
    }
    //
    // #[inline(always)]
    // pub fn create_font(&mut self, data: &'static [u8]) -> Result<Font, String> {
    //     // self.glyphs.create_font(data)
    //     let mut glyphs = &mut self.plugins.get_mut::<Draw, Draw2DPlugin>().unwrap().glyphs;
    //     glyphs.create_font(data)
    // }

    // #[cfg(feature = "glyphs")]
    // #[inline(always)]
    // pub fn update_glyphs(&mut self, render: &mut GlyphRenderer) -> Result<(), String> {
    //     self.glyphs.update(&mut self.device, render)
    // }

    // #[inline(always)]
    // pub fn process_text(&mut self, font: &Font, text: &Text) {
    //     self.glyphs.process_text(font, text);
    // }

    // #[inline(always)]
    // pub fn create_draw(&self) -> Draw {
    //     let (width, height) = self.device.size();
    //     self.draw.create_draw(width, height)
    // }

    // #[cfg(feature = "glyphs")]
    // #[inline(always)]
    // pub fn glyphs_texture(&self) -> &Texture {
    //     &self.glyphs.texture
    // }
    //
    // pub fn render_to<'a>(
    //     &mut self,
    //     target: &RenderTexture,
    //     render: impl Into<GraphicsRenderer<'a>>,
    // ) {
    //     let commands = match render.into() {
    //         GraphicsRenderer::Raw(r) => r,
    //         GraphicsRenderer::Device(r) => r.commands_from(&mut self.device),
    //         GraphicsRenderer::Draw(r) => {
    //             r.commands(&mut self.device, &mut self.draw, &mut self.glyphs)
    //         }
    //     };
    //     self.device.render_to(target, commands);
    // }
    //
    // pub fn render<'a>(&mut self, render: impl Into<GraphicsRenderer<'a>>) {
    //     let commands = match render.into() {
    //         GraphicsRenderer::Raw(r) => r,
    //         GraphicsRenderer::Device(r) => r.commands_from(&mut self.device),
    //         GraphicsRenderer::Draw(r) => {
    //             r.commands(&mut self.device, &mut self.draw, &mut self.glyphs)
    //         }
    //     };
    //     self.device.render(commands);
    // }

    pub fn r(&mut self, render: &GfxRenderer) {
        render.render(self);
    }

    // #[inline(always)]
    // pub fn create_draw_image_pipeline(
    //     &mut self,
    //     fragment: Option<&ShaderSource>,
    // ) -> Result<Pipeline, String> {
    //     self.draw.create_image_pipeline(&mut self.device, fragment)
    // }
    //
    // #[inline(always)]
    // pub fn create_draw_pattern_pipeline(
    //     &mut self,
    //     fragment: Option<&ShaderSource>,
    // ) -> Result<Pipeline, String> {
    //     self.draw
    //         .create_pattern_pipeline(&mut self.device, fragment)
    // }
    //
    // #[inline(always)]
    // pub fn create_draw_shape_pipeline(
    //     &mut self,
    //     fragment: Option<&ShaderSource>,
    // ) -> Result<Pipeline, String> {
    //     self.draw.create_shape_pipeline(&mut self.device, fragment)
    // }
    //
    // #[inline(always)]
    // pub fn create_draw_text_pipeline(
    //     &mut self,
    //     fragment: Option<&ShaderSource>,
    // ) -> Result<Pipeline, String> {
    //     self.draw.create_text_pipeline(&mut self.device, fragment)
    // }
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

pub enum GraphicsRenderer<'a> {
    Raw(&'a [Commands]),
    Device(&'a DeviceRenderer),
    // Draw(&'a DrawRenderer),
}

impl<'a> From<&'a [Commands]> for GraphicsRenderer<'a> {
    fn from(r: &'a [Commands]) -> GraphicsRenderer {
        GraphicsRenderer::Raw(r)
    }
}

impl<'a> From<&'a Renderer> for GraphicsRenderer<'a> {
    fn from(r: &'a Renderer) -> GraphicsRenderer {
        GraphicsRenderer::Device(r)
    }
}

// impl<'a> From<&'a Draw> for GraphicsRenderer<'a> {
//     fn from(r: &'a Draw) -> GraphicsRenderer {
//         GraphicsRenderer::Draw(r)
//     }
// }

// -
use downcast_rs::{impl_downcast, Downcast};
use hashbrown::HashMap;
use hecs::{DynamicBundle, Entity, World};
use indexmap::IndexMap;
use std::any::{Any, TypeId};
use std::cell::{RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct GfxPlugins {
    // map: HashMap<TypeId, Rc<dyn Any>>,
    world: World,
    entity: Entity,
}

impl Default for GfxPlugins {
    fn default() -> Self {
        let mut world = World::new();
        let entity = world.reserve_entity();
        Self { world, entity }
    }
}

impl GfxPlugins {
    pub fn set<R: GfxRenderer, T: GraphicPlugin<R> + 'static>(&mut self, value: T) {
        // self.map.insert(TypeId::of::<T>(), Rc::new(RefCell::new(value)));
        self.world.insert_one(self.entity, value);
    }

    // /// Returns the plugin of the type passed
    // pub fn get<R: GfxRenderer, T: GraphicPlugin<R> + 'static>(&self) -> Option<&T> {
    //     self.map
    //         .get(&TypeId::of::<T>())
    //         .map(|value| value.downcast_ref().unwrap())
    // }

    // /// Returns the plugin of the type passed as mutable reference
    pub fn get_mut<R: GfxRenderer, T: GraphicPlugin<R> + 'static>(
        &self,
    ) -> Option<hecs::RefMut<'_, T>> {
        self.world.get_mut(self.entity).ok()
    }
    //     // self.map
    //     //     .get(&TypeId::of::<T>())
    //     //     .map(|value| {
    //     //
    //     //         // let rc = value.as_any_mut().downcast_mut::<Rc<RefCell<T>>>().unwrap().clone();
    //     //         // let ref_m = RefMut::map(rc.borrow_mut(), |v| v);
    //     //         // let rc = value.clone().as_any().downcast::<Rc<RefCell<T>>>().unwrap();
    //     //         let rc = value.clone().downcast::<RefCell<T>>().unwrap();
    //     //
    //     //         let mut w = ExtWrapper {
    //     //             rc,
    //     //             ref_m: None,
    //     //         };
    //     //
    //     //         w
    //     //     })
    //     match self.map.get(&TypeId::of::<T>()).cloned() {
    //         Some(value) => {
    //             let rc = value.clone().downcast::<RefCell<T>>().unwrap();
    //
    //             let mut w = ExtWrapper {
    //                 rc,
    //                 ref_m: None,
    //             };
    //
    //             // w.ref_m = Some(RefMut::map(wrc.borrow_mut(), |v| v));
    //
    //             Some(w)
    //         }
    //         None => None,
    //     }
    // }
}

pub struct ExtWrapper<T: 'static> {
    rc: Rc<RefCell<T>>,
    ref_m: Option<RefMut<'static, T>>,
}
//
// impl<T> Deref for ExtWrapper<T> {
//     type Target = T;
//
//     fn deref(&self) -> &Self::Target {
//         self.ref_m.as_ref().unwrap()
//     }
// }
//
// impl<T> DerefMut for ExtWrapper<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         if self.ref_m.is_none() {
//             self.ref_m = Some(RefMut::map(self.rc.borrow_mut(), |v| v));
//         }
//         self.ref_m.as_mut().unwrap()
//     }
// }

pub trait GfxRenderer
where
    Self: Any + Downcast,
{
    fn render(&self, gfx: &mut Graphics);
}

pub trait GraphicPlugin<T: ?Sized>
where
    Self: Any + Send + Sync + Downcast,
{
    fn prepare<'a>(&'a mut self, device: &mut Device, renderer: &'a T) -> &'a [Commands];
}
//
//
// impl_downcast!(GfxRenderer);
// impl_downcast!(GraphicPlugin<GfxRenderer>);

impl GfxRenderer for Renderer {
    fn render(&self, gfx: &mut Graphics) {
        gfx.device.render(self.commands());
    }
}
//
// struct Draw2DPlugin {
//     pub manager: DrawManager,
//     // pub glyphs: GlyphManager
// }
//
// impl GraphicPlugin<Draw> for Draw2DPlugin {
//     fn prepare<'a>(&'a mut self, device: &mut Device, renderer: &'a Draw) -> &'a [Commands] {
//         // renderer.commands(device, &mut self.manager, &mut self.glyphs)
//         todo!()
//     }
// }
//
// impl GfxRenderer for Draw {
//     fn render(&self, gfx: &mut Graphics) {
//         let plugin= gfx.plugins.get_mut::<Self, Draw2DPlugin>().unwrap();
//         let commands = plugin.prepare(&mut gfx.device, self);
//         gfx.device.render(commands);
//     }
// }
