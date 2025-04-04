#![allow(clippy::type_complexity)]

use crate::epaint::Primitive;
use crate::plugin::Output;
use crate::TextureId;
use egui::load::SizedTexture;
use egui::{PaintCallbackInfo, Rect};
use notan_app::{
    BlendFactor, BlendMode, Buffer, CullMode, Device, Graphics, Pipeline, RenderTexture,
    ShaderSource, Texture, TextureFilter, TextureFormat, VertexFormat, VertexInfo,
};
use std::collections::HashMap;

//language=glsl
const EGUI_VERTEX: ShaderSource = notan_macro::vertex_shader! {
    r#"
    #version 450
    
    #ifdef GL_ES
        precision mediump float;
    #endif

    layout(location = 0) in vec2 a_pos;
    layout(location = 1) in vec2 a_tc;
    layout(location = 2) in vec4 a_srgba;

    layout(location = 0) out vec4 v_rgba_in_gamma;
    layout(location = 1) out vec2 v_tc;
    layout(location = 2) out float v_srgb_enabled;

    layout(set = 0, binding = 0) uniform Locals {
        vec2 u_screen_size;
        float srgb_enabled;
    };

    void main() {
        v_srgb_enabled = srgb_enabled;
        gl_Position = vec4(
            2.0 * a_pos.x / u_screen_size.x - 1.0,
            1.0 - 2.0 * a_pos.y / u_screen_size.y,
            0.0,
            1.0
        );

        v_rgba_in_gamma = a_srgba / 255.0;
        v_tc = a_tc;
    }
    "#
};

//language=glsl
const EGUI_FRAGMENT: ShaderSource = notan_macro::fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) in vec4 v_rgba_in_gamma;
    layout(location = 1) in vec2 v_tc;
    layout(location = 2) in float v_srgb_enabled;

    layout(location = 0) out vec4 color;

    layout(binding = 0) uniform sampler2D u_sampler;

    // 0-1 sRGB gamma  from  0-1 linear
    vec3 srgb_gamma_from_linear(vec3 rgb) {
        bvec3 cutoff = lessThan(rgb, vec3(0.0031308));
        vec3 lower = rgb * vec3(12.92);
        vec3 higher = vec3(1.055) * pow(rgb, vec3(1.0 / 2.4)) - vec3(0.055);
        return mix(higher, lower, vec3(cutoff));
    }

    // 0-1 sRGBA gamma  from  0-1 linear
    vec4 srgba_gamma_from_linear(vec4 rgba) {
        return vec4(srgb_gamma_from_linear(rgba.rgb), rgba.a);
    }

    void main() {
        vec4 texture_in_gamma = texture(u_sampler, v_tc);
        if (v_srgb_enabled == 1.0) {
            texture_in_gamma = srgba_gamma_from_linear(texture_in_gamma);
        }
        // Multiply vertex color with texture color (in linear space).
        color = v_rgba_in_gamma * texture_in_gamma;
    }
"#
};

pub struct EguiCallbackFn {
    f: Box<dyn Fn(PaintCallbackInfo, &mut Device) + Sync + Send>,
}

impl EguiCallbackFn {
    pub fn new<F: Fn(PaintCallbackInfo, &mut Device) + Sync + Send + 'static>(callback: F) -> Self {
        let f = Box::new(callback);
        EguiCallbackFn { f }
    }
}

pub struct EguiExtension {
    pipeline: Pipeline,
    vbo: Buffer,
    ebo: Buffer,
    ubo: Buffer,
    textures: HashMap<egui::TextureId, Texture>,
}

impl EguiExtension {
    pub fn new(gfx: &mut Graphics) -> Result<Self, String> {
        let vertex_info = VertexInfo::new()
            .attr(0, VertexFormat::Float32x2)
            .attr(1, VertexFormat::Float32x2)
            .attr(2, VertexFormat::UInt8x4);

        let pipeline = gfx
            .create_pipeline()
            .from(&EGUI_VERTEX, &EGUI_FRAGMENT)
            .with_vertex_info(&vertex_info)
            .with_color_blend(BlendMode::new(
                BlendFactor::One,
                BlendFactor::InverseSourceAlpha,
            ))
            .with_alpha_blend(BlendMode::new(
                BlendFactor::InverseDestinationAlpha,
                BlendFactor::One,
            ))
            .with_srgb_space(cfg!(target_arch = "wasm32"))
            .with_cull_mode(CullMode::None)
            .with_texture_location(0, "u_sampler")
            .build()?;

        let vbo = gfx.create_vertex_buffer().with_info(&vertex_info).build()?;

        let ebo = gfx.create_index_buffer().build()?;
        let ubo = gfx
            .create_uniform_buffer(0, "Locals")
            .with_data(&[0.0; 3])
            .build()?;

        let mut textures = HashMap::new();
        let fonts_texture = create_empty_texture(gfx, 0, 0)?;
        textures.insert(egui::TextureId::default(), fonts_texture);

        Ok(Self {
            pipeline,
            vbo,
            ebo,
            ubo,
            textures,
        })
    }

    pub fn add_texture(&mut self, texture: &Texture) -> SizedTexture {
        let id = egui::TextureId::User(texture.id());
        let size: egui::Vec2 = texture.size().into();
        self.textures.insert(id, texture.clone());
        SizedTexture { id, size }
    }

    pub fn remove_texture(&mut self, id: impl Into<TextureId>) {
        self.free_texture(id.into());
    }

    fn set_texture(
        &mut self,
        device: &mut Device,
        id: egui::TextureId,
        delta: &egui::epaint::ImageDelta,
    ) -> Result<(), String> {
        let [width, height] = delta.image.size();

        // update texture
        if let Some([x, y]) = delta.pos {
            let texture = self
                .textures
                .entry(id)
                .or_insert_with(|| create_empty_texture(device, width as _, height as _).unwrap());

            match &delta.image {
                egui::ImageData::Color(image) => {
                    debug_assert_eq!(
                        image.width() * image.height(),
                        image.pixels.len(),
                        "Mismatch between texture size and texel count"
                    );

                    let data = bytemuck::cast_slice(image.pixels.as_ref());
                    update_texture(
                        device,
                        texture,
                        data,
                        x as _,
                        y as _,
                        width as _,
                        height as _,
                    )?
                }
                egui::ImageData::Font(image) => {
                    debug_assert_eq!(
                        image.width() * image.height(),
                        image.pixels.len(),
                        "Mismatch between texture size and texel count"
                    );

                    let data: Vec<u8> = image
                        .srgba_pixels(None)
                        .flat_map(|a| a.to_array())
                        .collect();

                    update_texture(
                        device,
                        texture,
                        &data,
                        x as _,
                        y as _,
                        width as _,
                        height as _,
                    )?
                }
            }

            return Ok(());
        }

        // create a new texture
        let texture = match &delta.image {
            egui::ImageData::Color(image) => {
                debug_assert_eq!(
                    image.width() * image.height(),
                    image.pixels.len(),
                    "Mismatch between texture size and texel count"
                );

                let data = bytemuck::cast_slice(image.pixels.as_ref());
                create_texture(device, data, width as _, height as _)?
            }
            egui::ImageData::Font(image) => {
                debug_assert_eq!(
                    image.width() * image.height(),
                    image.pixels.len(),
                    "Mismatch between texture size and texel count"
                );

                let data: Vec<u8> = image
                    .srgba_pixels(None)
                    .flat_map(|a| a.to_array())
                    .collect();

                create_texture(device, &data, width as _, height as _)?
            }
        };

        self.textures.insert(id, texture);
        Ok(())
    }

    fn free_texture(&mut self, tex_id: egui::TextureId) {
        self.textures.remove(&tex_id);
    }

    pub(crate) fn paint_and_update_textures(
        &mut self,
        device: &mut Device,
        meshes: Vec<egui::ClippedPrimitive>,
        textures_delta: &egui::TexturesDelta,
        target: Option<&RenderTexture>,
        zoom_factor: f32,
    ) -> Result<(), String> {
        for (id, image_delta) in &textures_delta.set {
            self.set_texture(device, *id, image_delta)?;
        }

        self.paint_primitives(device, meshes, target, zoom_factor)?;

        for &id in &textures_delta.free {
            self.free_texture(id);
        }

        Ok(())
    }

    fn paint_primitives(
        &mut self,
        device: &mut Device,
        meshes: Vec<egui::ClippedPrimitive>,
        target: Option<&RenderTexture>,
        zoom_factor: f32,
    ) -> Result<(), String> {
        let (width, height) = target.map_or(device.size(), |rt| {
            (rt.base_width() as _, rt.base_height() as _)
        });

        for egui::ClippedPrimitive {
            clip_rect,
            primitive,
        } in &meshes
        {
            match primitive {
                Primitive::Mesh(mesh) => {
                    self.paint_mesh(
                        device,
                        zoom_factor * (*clip_rect),
                        mesh,
                        target,
                        zoom_factor,
                    )?;
                }
                Primitive::Callback(callback) => {
                    let rect = Rect {
                        min: callback.rect.min,
                        max: clip_rect.max.min(callback.rect.max),
                    };

                    if callback.rect.is_positive() {
                        let info = egui::PaintCallbackInfo {
                            viewport: zoom_factor * callback.rect,
                            clip_rect: zoom_factor * rect,
                            pixels_per_point: device.dpi() as _,
                            screen_size_px: [width as _, height as _],
                        };

                        match callback.callback.downcast_ref::<EguiCallbackFn>() {
                            Some(callback) => (callback.f)(info, device),
                            None => {
                                log::warn!("Warning: Unsupported render callback. Expected notan_egui::CallbackFn");
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn paint_mesh(
        &mut self,
        device: &mut Device,
        clip_rect: egui::Rect,
        primitive: &egui::Mesh,
        target: Option<&RenderTexture>,
        zoom_factor: f32,
    ) -> Result<(), String> {
        let (width_in_pixels, height_in_pixels) = target.map_or(device.size(), |rt| {
            (rt.base_width() as _, rt.base_height() as _)
        });

        let texture = self
            .textures
            .get(&primitive.texture_id)
            .ok_or_else(|| format!("Invalid EGUI texture id {:?}", &primitive.texture_id))?;

        let is_srgb_texture = matches!(texture.format(), TextureFormat::SRgba8);
        let srgb_enabled = cfg!(target_arch = "wasm32") && is_srgb_texture;
        let srgb_as_float = if srgb_enabled { 1.0 } else { 0.0 };
        let uniforms: [f32; 3] = [
            (width_in_pixels as f32) / zoom_factor,
            (height_in_pixels as f32) / zoom_factor,
            srgb_as_float,
        ];
        device.set_buffer_data(&self.ubo, &uniforms);

        let vertices: &[f32] = bytemuck::cast_slice(&primitive.vertices);
        device.set_buffer_data(&self.vbo, vertices);
        device.set_buffer_data(&self.ebo, &primitive.indices);

        let clip_min_x = clip_rect.min.x;
        let clip_min_y = clip_rect.min.y;
        let clip_max_x = clip_rect.max.x;
        let clip_max_y = clip_rect.max.y;

        // Make sure clip rect can fit within a `u32`:
        let clip_min_x = clip_min_x.clamp(0.0, width_in_pixels as _);
        let clip_min_y = clip_min_y.clamp(0.0, height_in_pixels as _);
        let clip_max_x = clip_max_x.clamp(clip_min_x, width_in_pixels as _);
        let clip_max_y = clip_max_y.clamp(clip_min_y, height_in_pixels as _);

        let clip_min_x = clip_min_x.round();
        let clip_min_y = clip_min_y.round();
        let clip_max_x = clip_max_x.round();
        let clip_max_y = clip_max_y.round();

        let width = clip_max_x - clip_min_x;
        let height = clip_max_y - clip_min_y;

        // render pass
        let mut renderer = device.create_renderer();
        renderer.set_scissors(clip_min_x, clip_min_y, width, height);
        renderer.begin(None);
        renderer.set_pipeline(&self.pipeline);
        renderer.bind_buffers(&[&self.vbo, &self.ebo, &self.ubo]);
        renderer.bind_texture(0, texture);
        renderer.draw(0, primitive.indices.len() as _);
        renderer.end();

        match target {
            Some(rt) => device.render_to(rt, renderer.commands()),
            _ => device.render(renderer.commands()),
        }

        Ok(())
    }
}

pub trait EguiRegisterTexture {
    fn egui_register_texture(&mut self, texture: &Texture) -> egui::load::SizedTexture;
    fn egui_remove_texture(&mut self, id: impl Into<egui::TextureId>);
}

impl EguiRegisterTexture for Graphics {
    fn egui_register_texture(&mut self, texture: &Texture) -> SizedTexture {
        self.extension_mut::<Output, EguiExtension>()
            .unwrap()
            .add_texture(texture)
    }

    fn egui_remove_texture(&mut self, id: impl Into<TextureId>) {
        self.extension_mut::<Output, EguiExtension>()
            .unwrap()
            .remove_texture(id);
    }
}

#[inline]
fn create_texture(
    device: &mut Device,
    data: &[u8],
    width: u32,
    height: u32,
) -> Result<Texture, String> {
    let texture_format = if cfg!(target_arch = "wasm32") {
        TextureFormat::SRgba8
    } else {
        TextureFormat::Rgba32
    };

    let texture_filter = if cfg!(target_arch = "wasm32") {
        TextureFilter::Linear
    } else {
        TextureFilter::Nearest
    };

    device
        .create_texture()
        .from_bytes(data, width, height)
        .with_format(texture_format)
        .with_filter(texture_filter, texture_filter)
        .build()
}

#[inline]
fn create_empty_texture(device: &mut Device, width: u32, height: u32) -> Result<Texture, String> {
    let texture_format = if cfg!(target_arch = "wasm32") {
        TextureFormat::SRgba8
    } else {
        TextureFormat::Rgba32
    };

    let texture_filter = if cfg!(target_arch = "wasm32") {
        TextureFilter::Linear
    } else {
        TextureFilter::Nearest
    };

    device
        .create_texture()
        .from_empty_buffer(width, height)
        .with_format(texture_format)
        .with_filter(texture_filter, texture_filter)
        .build()
}

#[inline]
fn update_texture(
    device: &mut Device,
    texture: &mut Texture,
    data: &[u8],
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<(), String> {
    device
        .update_texture(texture)
        .with_data(data)
        .with_x_offset(x)
        .with_y_offset(y)
        .with_width(width)
        .with_height(height)
        .update()
}
