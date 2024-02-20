use notan_gfx::{NotanSampler, NotanTexture, Sampler, Texture};
use notan_math::{vec2, Vec2};

// TODO frame and texture_base
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SpriteId {
    texture_id: u64,
    sampler_id: u64,
}

#[derive(Clone)]
pub struct Sprite {
    id: SpriteId,
    texture: Texture,
    sampler: Sampler,
    size: Vec2,
}

impl Sprite {
    pub fn new(texture: Texture, sampler: Sampler) -> Self {
        let id = SpriteId {
            texture_id: texture.id().into(),
            sampler_id: sampler.id().into(),
        };
        let size = vec2(texture.width() as _, texture.height() as _);
        Self {
            id,
            texture,
            sampler,
            size,
        }
    }

    pub fn id(&self) -> SpriteId {
        self.id
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn width(&self) -> f32 {
        self.size.x
    }

    pub fn height(&self) -> f32 {
        self.size.y
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn sampler(&self) -> &Sampler {
        &self.sampler
    }
}

impl PartialEq for Sprite {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
