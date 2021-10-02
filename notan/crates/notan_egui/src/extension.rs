use crate::context::EguiContext;
use crate::Color32;
use egui::TextureId;
use notan_app::{
    BlendFactor, BlendMode, BufferBuildImpl, ClearOptions, Color, Commands, Device, ExtContainer,
    GfxExtension, GfxRenderer, Graphics, IndexBuffer, Pipeline, RenderTexture, ShaderSource,
    Texture, TextureFilter, TextureFormat, TextureInfo, UniformBuffer, VertexBuffer, VertexFormat,
};

//language=glsl
const EGUI_VERTEX: ShaderSource = notan_macro::vertex_shader! {
    r#"
    #version 450

    layout(location = 0) in vec2 a_pos;
    layout(location = 1) in vec2 a_tc;
    layout(location = 2) in vec4 a_srgba;

    layout(location = 0) out vec4 v_rgba;
    layout(location = 1) out vec2 v_tc;

    layout(set = 0, binding = 0) uniform Locals {
        vec2 u_screen_size;
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
        gl_Position = vec4(
            2.0 * a_pos.x / u_screen_size.x - 1.0,
            1.0 - 2.0 * a_pos.y / u_screen_size.y,
            0.0,
            1.0
        );

        // notan only support f32 vbo (right now), we need to convert this to bytes
        vec4 norm_srgba = a_srgba * vec4(255, 255, 255, 255);

        // egui encodes vertex colors in gamma spaces, so we must decode the colors here:
        v_rgba = linear_from_srgba(norm_srgba);
        v_tc = a_tc;
    }
    "#
};

//language=glsl
const EGUI_FRAGMENT: ShaderSource = notan_macro::fragment_shader! {
    r#"
    #version 450

    layout(location = 0) in vec4 v_rgba;
    layout(location = 1) in vec2 v_tc;

    layout(set = 0, binding = 0) uniform sampler2D u_sampler;

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
        // The texture is set up with `SRGB8_ALPHA8`, so no need to decode here!
        vec4 texture_rgba = texture(u_sampler, v_tc);

        /// Multiply vertex color with texture color (in linear space).
        color = v_rgba * texture_rgba;

        // We must gamma-encode again since WebGL doesn't support linear blending in the framebuffer.
        color = srgba_from_linear(v_rgba * texture_rgba) / 255.0;

        // WebGL doesn't support linear blending in the framebuffer,
        // so we apply this hack to at least get a bit closer to the desired blending:
        color.a = pow(color.a, 1.6); // Empiric nonsense
    }
"#
};

pub struct EguiExtension {
    pipeline: Pipeline,
    vbo: VertexBuffer,
    ebo: IndexBuffer,
    ubo: UniformBuffer,
    texture: Option<Texture>,
    texture_version: Option<u64>,
    user_textures: Vec<Option<Texture>>,
}

impl EguiExtension {
    pub fn new(gfx: &mut Graphics) -> Result<Self, String> {
        let pipeline = gfx
            .create_pipeline()
            .from(&EGUI_VERTEX, &EGUI_FRAGMENT)
            .vertex_attr(0, VertexFormat::Float2)
            .vertex_attr(1, VertexFormat::Float2)
            .vertex_attr(2, VertexFormat::Float4)
            .with_color_blend(BlendMode::new(
                BlendFactor::One,
                BlendFactor::InverseSourceAlpha,
            ))
            .build()?;

        let vbo = gfx.create_vertex_buffer().build()?;
        let ebo = gfx.create_index_buffer().build()?;
        let ubo = gfx
            .create_uniform_buffer(0, "Locals")
            .with_data(vec![0.0; 2])
            .build()?;

        Ok(Self {
            pipeline,
            vbo,
            ebo,
            ubo,
            texture: None,
            texture_version: None,
            user_textures: vec![],
        })
    }

    #[inline]
    fn build_texture(&mut self, device: &mut Device, egui_tex: &egui::Texture) {
        if self.texture_version == Some(egui_tex.version) {
            return; // No change
        }

        let width = egui_tex.width;
        let height = egui_tex.height;

        let font_gamma = if cfg!(target_arch = "wasm32") {
            1.0 / 2.2 // HACK due to non-linear framebuffer blending.
        } else {
            1.0
        };

        let pixels = egui_tex
            .srgba_pixels(font_gamma)
            .flat_map(|c| c.to_array())
            .collect::<Vec<u8>>();

        let texture = device
            .create_texture(TextureInfo {
                width: width as _,
                height: height as _,
                format: TextureFormat::Rgba,
                internal_format: TextureFormat::Rgba,
                min_filter: TextureFilter::Linear,
                mag_filter: TextureFilter::Linear,
                bytes: Some(pixels),
                depth: false,
            })
            .unwrap();

        self.texture = Some(texture);
        self.texture_version = Some(egui_tex.version);
    }

    fn paint_meshes(
        &mut self,
        device: &mut Device,
        meshes: Vec<egui::ClippedMesh>,
        egui_tex: &egui::Texture,
        target: Option<&RenderTexture>,
    ) {
        self.build_texture(device, egui_tex);

        let (width, height) = device.size();
        self.ubo.set(&[width as _, height as _]);

        meshes
            .iter()
            .for_each(|egui::ClippedMesh(clip_rect, mesh)| {
                self.paint_mesh(device, *clip_rect, mesh, target)
            });
    }

    fn paint_mesh(
        &mut self,
        device: &mut Device,
        clip_rect: egui::Rect,
        mesh: &egui::paint::Mesh,
        target: Option<&RenderTexture>,
    ) {
        let vertices: Vec<f32> = mesh
            .vertices
            .iter()
            .flat_map(|v| {
                let color: Color = v.color.to_array().into();
                [
                    v.pos.x, v.pos.y, v.uv.x, v.uv.y, color.r, color.g, color.b, color.a,
                ]
            })
            .collect();

        self.vbo.set(&vertices);
        self.ebo.set(&mesh.indices);

        let pixels_per_point = device.dpi() as f32;
        let (width_in_pixels, height_in_pixels) = device.size();

        let clip_min_x = pixels_per_point * clip_rect.min.x;
        let clip_min_y = pixels_per_point * clip_rect.min.y;
        let clip_max_x = pixels_per_point * clip_rect.max.x;
        let clip_max_y = pixels_per_point * clip_rect.max.y;

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

        let texture = self.texture.as_ref().unwrap();

        let mut renderer = device.create_renderer();
        renderer.begin(None);
        renderer.set_pipeline(&self.pipeline);
        renderer.bind_index_buffer(&self.ebo);
        renderer.bind_vertex_buffer(&self.vbo);
        renderer.bind_uniform_buffer(&self.ubo);
        renderer.bind_texture(0, texture);
        renderer.set_scissors(clip_min_x, clip_min_y, width, height);
        renderer.draw(0, mesh.indices.len() as _);
        renderer.end();

        match target {
            Some(rt) => device.render_to(rt, renderer.commands()),
            _ => device.render(renderer.commands()),
        }
    }

    // TODO https://github.com/emilk/egui/blob/master/egui_glium/src/painter.rs#L231
    pub fn get_texture(&self, tex_id: egui::TextureId) -> Option<&Texture> {
        match tex_id {
            TextureId::Egui => self.texture.as_ref(),
            TextureId::User(id) => self.user_textures.get(id as usize)?.as_ref(),
        }
    }
}

impl GfxRenderer for EguiContext {
    fn render(
        &self,
        device: &mut Device,
        extensions: &mut ExtContainer,
        target: Option<&RenderTexture>,
    ) {
        let (_output, shapes) = self.ctx.end_frame();

        // if output.needs_repaint { // FIXME this doesn't work if the user is doing a clear between frames
        let meshes = self.ctx.tessellate(shapes);
        let texture = self.ctx.texture();

        if self.clear_color.is_some() {
            let mut clear_renderer = device.create_renderer();
            clear_renderer.begin(Some(&ClearOptions {
                color: self.clear_color,
                ..Default::default()
            }));
            clear_renderer.end();

            match target {
                Some(rt) => device.render_to(rt, clear_renderer.commands()),
                _ => device.render(clear_renderer.commands()),
            }
        }

        let mut plugin = extensions.get_mut::<Self, EguiExtension>().unwrap();
        plugin.paint_meshes(device, meshes, &texture, target);
        // }
    }
}

impl GfxExtension<EguiContext> for EguiExtension {
    fn commands<'a>(
        &'a mut self,
        _device: &mut Device,
        _renderer: &'a EguiContext,
    ) -> &'a [Commands] {
        &[]
    }
}

pub trait EguiColorConversion {
    fn to_egui(&self) -> egui::Color32;
    fn to_notan(&self) -> Color;
}

impl EguiColorConversion for Color {
    fn to_egui(&self) -> Color32 {
        let [r, g, b, a] = self.rgba_u8();
        Color32::from_rgba_premultiplied(r, g, b, a)
    }

    fn to_notan(&self) -> Color {
        *self
    }
}

impl EguiColorConversion for Color32 {
    fn to_egui(&self) -> Color32 {
        *self
    }

    fn to_notan(&self) -> Color {
        self.to_array().into()
    }
}
