use crate::draw2::Draw2;
use notan_graphics::prelude::*;

#[derive(Default, Clone, Debug)]
pub(crate) struct CustomPipeline {
    pub pipeline: Option<Pipeline>,
    pub uniforms: Option<Vec<Buffer<f32>>>,
}

impl CustomPipeline {
    fn clear(&mut self) {
        self.pipeline = None;
        self.uniforms = None;
    }
}

impl std::cmp::PartialEq for CustomPipeline {
    fn eq(&self, other: &Self) -> bool {
        self.pipeline == other.pipeline && self.uniforms == other.uniforms
    }
}

pub trait DrawCustomPipeline {
    fn image_pipeline(&mut self) -> CustomPipelineBuilder;
    fn shape_pipeline(&mut self) -> CustomPipelineBuilder;
    fn pattern_pipeline(&mut self) -> CustomPipelineBuilder;
    fn text_pipeline(&mut self) -> CustomPipelineBuilder;
}

impl DrawCustomPipeline for Draw2 {
    fn image_pipeline(&mut self) -> CustomPipelineBuilder {
        CustomPipelineBuilder::new(self, CustomPipelineType::Image)
    }

    fn shape_pipeline(&mut self) -> CustomPipelineBuilder {
        CustomPipelineBuilder::new(self, CustomPipelineType::Shape)
    }

    fn pattern_pipeline(&mut self) -> CustomPipelineBuilder {
        CustomPipelineBuilder::new(self, CustomPipelineType::Pattern)
    }

    fn text_pipeline(&mut self) -> CustomPipelineBuilder {
        CustomPipelineBuilder::new(self, CustomPipelineType::Text)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CustomPipelineType {
    Image,
    Shape,
    Pattern,
    Text,
}

pub struct CustomPipelineBuilder<'a> {
    draw: &'a mut Draw2,
    typ: CustomPipelineType,
    pipeline: Option<&'a Pipeline>,
    uniforms: Option<Vec<&'a Buffer<f32>>>,
    removing: bool,
}

impl<'a> CustomPipelineBuilder<'a> {
    fn new(draw: &'a mut Draw2, typ: CustomPipelineType) -> Self {
        CustomPipelineBuilder {
            draw,
            typ,
            pipeline: None,
            uniforms: None,
            removing: false,
        }
    }

    pub fn pipeline(&mut self, pipeline: &'a Pipeline) -> &mut Self {
        self.pipeline = Some(pipeline);
        self
    }

    pub fn uniform_buffer(&mut self, buffer: &'a Buffer<f32>) -> &mut Self {
        let uniforms = self.uniforms.get_or_insert(vec![]);
        uniforms.push(buffer);
        self
    }

    pub fn remove(&mut self) {
        self.removing = true;
    }
}

impl Drop for CustomPipelineBuilder<'_> {
    fn drop(&mut self) {
        if self.removing {
            remove_pipeline(self.draw, self.typ);
            return;
        }

        process_pipeline(self, self.typ);
    }
}

fn process_pipeline(builder: &mut CustomPipelineBuilder, typ: CustomPipelineType) {
    let pip = match builder.pipeline.take() {
        Some(pip) => pip,
        _ => return,
    };

    let custom = get_custom_pipeline(builder.draw, typ);
    let needs_update = match &custom.pipeline {
        Some(c_pip) => *c_pip != *pip,
        _ => true,
    };

    if !needs_update {
        return;
    }

    custom.pipeline = Some(pip.clone());
    custom.uniforms = builder
        .uniforms
        .take()
        .map(|u| u.into_iter().cloned().collect::<Vec<_>>());
}

fn remove_pipeline(draw: &mut Draw2, typ: CustomPipelineType) {
    get_custom_pipeline(draw, typ).clear();
}

fn get_custom_pipeline(draw: &mut Draw2, typ: CustomPipelineType) -> &mut CustomPipeline {
    match typ {
        CustomPipelineType::Image => &mut draw.image_pipeline,
        CustomPipelineType::Shape => &mut draw.shape_pipeline,
        CustomPipelineType::Pattern => &mut draw.pattern_pipeline,
        CustomPipelineType::Text => &mut draw.text_pipeline,
    }
}
