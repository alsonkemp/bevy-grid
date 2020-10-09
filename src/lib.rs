extern crate bevy;

use bevy::prelude::*;
use bevy::sprite::QUAD_HANDLE;
use bevy::render::pipeline::{RenderPipeline, PipelineSpecialization, DynamicBinding};
use bevy::ui::UI_PIPELINE_HANDLE;

#[derive(Bundle, Clone)]
pub struct GridComponents {
    pub node: Node,
    pub style: Style,
    pub mesh: Handle<Mesh>, // TODO: maybe abstract this out
    pub material: Handle<ColorMaterial>,
    pub draw: Draw,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
impl Default for GridComponents {
    fn default() -> Self {
        Self {
            mesh: QUAD_HANDLE,
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
                UI_PIPELINE_HANDLE,
                PipelineSpecialization {
                    dynamic_bindings: vec![
                        // Transform
                        DynamicBinding {
                            bind_group: 1,
                            binding: 0,
                        },
                        // Node_size
                        DynamicBinding {
                            bind_group: 1,
                            binding: 1,
                        },
                    ],
                    ..Default::default()
                },
            )]),
            node: Default::default(),
            style: Default::default(),
            material: Default::default(),
            draw: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}
impl GridComponents {
    pub fn new() -> Self {
        Self {
            style: Style {
                size: Size::new(Val::Percent(48.0), Val::Percent(48.0)),
                border: Rect::all(Val::Percent(1.0)),
                margin: Rect::all(Val::Percent(1.0)),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
