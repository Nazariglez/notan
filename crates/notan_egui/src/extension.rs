use crate::epaint::Primitive;
use crate::plugin::Output;
use crate::TextureId;
use egui::Rect;
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

    layout(location = 0) out vec4 v_rgba;
    layout(location = 1) out vec2 v_tc;
    layout(location = 2) out float v_need_gamma_fix;

    layout(set = 0, binding = 0) uniform Locals {
        vec2 u_screen_size;
        float u_need_gamma_fix;
    };

    // 0-1 linear  from  0-255 sRGB
    vec3 linear_from_srgb(vec3 srgb) {
        bvec3 cutoff = lessThan(srgb, vec3(10.31475));
        vec3 lower = srgb / vec3(3294.6);
        vec3 higher = pow((srgb + vec3(14.025)) / vec3(269.025), vec3(2.4));
        return mix(higher, lower, vec3(cutoff));
    }

    vec4 linear_from_srgba(vec4 srgba) {
        return vec4(linear_from_srgb(srgba.rgb), srgba.a / 255.0);
    }

    void main() {
        v_need_gamma_fix = u_need_gamma_fix;

        gl_Position = vec4(
            2.0 * a_pos.x / u_screen_size.x - 1.0,
            1.0 - 2.0 * a_pos.y / u_screen_size.y,
            0.0,
            1.0
        );

        // egui encodes vertex colors in gamma spaces, so we must decode the colors here:
        v_rgba = linear_from_srgba(a_srgba);
        v_tc = a_tc;
    }
    "#
};

//language=glsl
const EGUI_FRAGMENT: ShaderSource = notan_macro::fragment_shader! {
    r#"
    #version 450
    
    #ifdef GL_ES
        precision mediump float;
    #endif

    layout(location = 0) in vec4 v_rgba;
    layout(location = 1) in vec2 v_tc;
    layout(location = 2) in float v_need_gamma_fix;

    layout(binding = 0) uniform sampler2D u_sampler;

    layout(location = 0) out vec4 color;

    // 0-255 sRGB  from  0-1 linear
    vec3 srgb_from_linear(vec3 rgb) {
        bvec3 cutoff = lessThan(rgb, vec3(0.0031308));
        vec3 lower = rgb * vec3(3294.6);
        vec3 higher = vec3(269.025) * pow(rgb, vec3(1.0 / 2.4)) - vec3(14.025);
        return mix(higher, lower, vec3(cutoff));
    }

    vec4 srgba_from_linear(vec4 rgba) {
        return vec4(srgb_from_linear(rgba.rgb), 255.0 * rgba.a);
    }

    void main() {
        vec4 texture_rgba = texture(u_sampler, v_tc);
        // Multiply vertex color with texture color (in linear space).
        color = v_rgba * texture_rgba;
        
        if (v_need_gamma_fix == 1.0) {
            if (color.a > 0.0) {
                color.rgb /= color.a;
            }
            
            color.a *= sqrt(color.a);
            
            // We must gamma-encode again since WebGL doesn't support linear blending in the framebuffer.
            color = srgba_from_linear(color) / 255.0;
            
            if (color.a > 0.0) {
                color.rgb *= color.a;
            }
        }
    }
"#
};

pub struct EguiExtension {
    pipeline: Pipeline,
    vbo: Buffer,
    ebo: Buffer,
    ubo: Buffer,
    textures: HashMap<egui::TextureId, Texture>,
    need_gamma_fix: bool,
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
            .with_cull_mode(CullMode::None)
            .with_texture_location(0, "u_sampler")
            .build()?;

        let vbo = gfx.create_vertex_buffer().with_info(&vertex_info).build()?;

        let ebo = gfx.create_index_buffer().build()?;
        let ubo = gfx
            .create_uniform_buffer(0, "Locals")
            .with_data(&[0.0; 3])
            .build()?;

        Ok(Self {
            pipeline,
            vbo,
            ebo,
            ubo,
            textures: HashMap::new(),
            need_gamma_fix: false,
        })
    }

    pub fn add_texture(&mut self, texture: &Texture) -> egui::TextureId {
        let id = egui::TextureId::User(texture.id());
        self.textures.insert(id, texture.clone());
        id
    }

    pub fn remove_texture(&mut self, id: egui::TextureId) {
        self.free_texture(id);
    }

    fn set_texture(
        &mut self,
        device: &mut Device,
        id: egui::TextureId,
        delta: &egui::epaint::ImageDelta,
    ) -> Result<(), String> {
        let [width, height] = delta.image.size();

        // todo mobile
        let gamma = if cfg!(target_arch = "wasm32") {
            1.0 / 2.2
        } else {
            1.0
        };

        // update texture
        if let Some([x, y]) = delta.pos {
            let texture = self
                .textures
                .get_mut(&id)
                .ok_or_else(|| format!("Failed to find EGUI texture {:?}", id))?;

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
                        .srgba_pixels(gamma)
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
                    .srgba_pixels(gamma)
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
    ) -> Result<(), String> {
        for (id, image_delta) in &textures_delta.set {
            self.set_texture(device, *id, image_delta)?;
        }

        self.paint_primitives(device, meshes, target)?;

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
    ) -> Result<(), String> {
        let (width, height) = target.map_or(device.size(), |rt| {
            (rt.base_width() as _, rt.base_height() as _)
        });

        self.need_gamma_fix = false;
        let uniforms: [f32; 3] = [width as _, height as _, 0.0];
        device.set_buffer_data(&self.ubo, &uniforms);

        for egui::ClippedPrimitive {
            clip_rect,
            primitive,
        } in &meshes
        {
            match primitive {
                Primitive::Mesh(mesh) => {
                    self.paint_mesh(device, *clip_rect, mesh, target)?;
                }
                Primitive::Callback(callback) => {
                    let rect = Rect {
                        min: callback.rect.min,
                        max: clip_rect.max.min(callback.rect.max),
                    };

                    if callback.rect.is_positive() {
                        let info = egui::PaintCallbackInfo {
                            viewport: callback.rect,
                            clip_rect: rect,
                            pixels_per_point: device.dpi() as _,
                            screen_size_px: [width as _, height as _],
                        };
                        callback.call(&info, device);
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
    ) -> Result<(), String> {
        let vertices: &[f32] = bytemuck::cast_slice(&primitive.vertices);
        device.set_buffer_data(&self.vbo, vertices);
        device.set_buffer_data(&self.ebo, &primitive.indices);

        let (width_in_pixels, height_in_pixels) = target.map_or(device.size(), |rt| {
            (rt.base_width() as _, rt.base_height() as _)
        });

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

        let texture = self
            .textures
            .get(&primitive.texture_id)
            .ok_or_else(|| format!("Invalid EGUI texture id {:?}", &primitive.texture_id))?;

        // webgl2 doesn't have a gl.enable(GL_FRAMEBUFFER_SRGB) so gamma should be fixed by fragment shader
        // to do that a float 0.0 - 1.0 is passed to the shader to indicate if it should or not encode gamma
        if cfg!(target_arch = "wasm32") {
            let is_srgb = matches!(texture.format(), TextureFormat::SRgba8);
            let ww: f32 = width_in_pixels as _;
            let hh: f32 = height_in_pixels as _;

            if is_srgb && !self.need_gamma_fix {
                self.need_gamma_fix = true;
                device.set_buffer_data(&self.ubo, &[ww, hh, 1.0]);
            } else if !is_srgb && self.need_gamma_fix {
                self.need_gamma_fix = false;
                device.set_buffer_data(&self.ubo, &[ww, hh, 0.0]);
            }
        }

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
    fn egui_register_texture(&mut self, texture: &Texture) -> egui::TextureId;
    fn egui_remove_texture(&mut self, id: egui::TextureId);
}

impl EguiRegisterTexture for Graphics {
    fn egui_register_texture(&mut self, texture: &Texture) -> TextureId {
        self.extension_mut::<Output, EguiExtension>()
            .unwrap()
            .add_texture(texture)
    }

    fn egui_remove_texture(&mut self, id: TextureId) {
        self.extension_mut::<Output, EguiExtension>()
            .unwrap()
            .remove_texture(id);
    }
}

#[inline]
fn create_texture(
    device: &mut Device,
    data: &[u8],
    width: i32,
    height: i32,
) -> Result<Texture, String> {
    device
        .create_texture()
        .from_bytes(data, width, height)
        .with_format(TextureFormat::SRgba8)
        .with_filter(TextureFilter::Linear, TextureFilter::Linear)
        .build()
}

#[inline]
fn update_texture(
    device: &mut Device,
    texture: &mut Texture,
    data: &[u8],
    x: i32,
    y: i32,
    width: i32,
    height: i32,
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
