use bevy::{asset, prelude::*, render::{pipeline::{PipelineDescriptor, RenderPipelines}, render_graph::{AssetRenderResourcesNode, RenderGraph, RenderResourcesNode, base}, shader::{ShaderStage, ShaderStages}, texture::TextureDimension}};
use bevy::render::renderer::RenderResources;
use bevy::reflect::TypeUuid;
use bevy::render::texture::Texture;
use bevy::render::pipeline::CullMode;
use bevy::render::pipeline::RenderPipeline;

use std::iter;


pub struct SkyboxPluginState {
    pub cube: Handle<Mesh>,
    pub pipeline: Handle<PipelineDescriptor>,
}

// Custom material for our custom shader
#[derive(RenderResources, TypeUuid)]
#[uuid = "9f84d5ec-9307-11eb-a8b3-0242ac130003"]
pub struct RenderableSkybox
{
    pub cubemap_texture: Handle<Texture>,
}

#[derive(TypeUuid, Default)]
#[uuid = "07843783-e2fc-4915-ac81-8fd6b075d93d"]
pub struct Skybox6Sided
{
    pub left: Handle<Texture>,
    pub right: Handle<Texture>,
    pub up: Handle<Texture>,
    pub down: Handle<Texture>,
    pub front: Handle<Texture>,
    pub back: Handle<Texture>
}

impl Skybox6Sided 
{
    pub fn new(asset_server: Res<AssetServer>, left: &str, right: &str, up: &str, down: &str, front: &str, back: &str) -> Self {
        Self {
            left: asset_server.load(left),
            right: asset_server.load(right),
            up: asset_server.load(up),
            down: asset_server.load(down),
            front: asset_server.load(front),
            back: asset_server.load(back),
        }
    }

    pub fn textures(&self) -> impl Iterator<Item = Handle<Texture>> {
        iter::once(self.left.clone())
            .chain(iter::once(self.right.clone()))
            .chain(iter::once(self.up.clone()))
            .chain(iter::once(self.down.clone()))
            .chain(iter::once(self.front.clone()))
            .chain(iter::once(self.back.clone()))
    }
}

pub const VERTEX_SHADER: &str = r#"
#version 450
layout(location = 0) in vec3 Vertex_Position;

layout(location = 0) out vec3 v_Direction;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
/*
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
*/
void main() {
    v_Direction = Vertex_Position;
    gl_Position = ViewProj * vec4(Vertex_Position, 1.0);
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
#version 450
layout(location = 0) in  vec3 v_Direction;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform texture2DArray RenderableSkybox_cubemap_texture;
layout(set = 2, binding = 1) uniform sampler RenderableSkybox_cubemap_texture_sampler;

void main() {
    o_Target = texture(sampler2DArray(RenderableSkybox_cubemap_texture, RenderableSkybox_cubemap_texture_sampler), v_Direction);
}
"#;

pub fn create_renderable_skybox(
    commands: &mut Commands,
    mut textures: ResMut<Assets<Texture>>,
    skybox_state: Res<SkyboxPluginState>,
    loading_skyboxes: Query<(Entity, &Skybox6Sided), (Without<RenderableSkybox>,)>,
) {
    for (entity, skybox_6_sided) in loading_skyboxes.iter() {
        let six_texture_handles_iter = skybox_6_sided.textures();
        let six_textures_option: Option<Vec<_>> = six_texture_handles_iter.map(|texture_handle| textures.get(texture_handle)).collect();
        if let Some(six_textures) = six_textures_option {
            assert!(six_textures.windows(2).all(|texture_pair| texture_pair[0].size == texture_pair[1].size), 
                "All textures of a 6 sided skybox should be an equal size!");
            assert!(six_textures.iter().all(|texture| texture.dimension == TextureDimension::D2), 
                "Skybox6Sided is supposed to have 6 2d images attached, not another format");
            println!("6_sided skybox all textures loaded!!!");
            
            let array_texture_data: Vec<_> = six_textures.iter().map(|texture| texture.data.clone()).flatten().collect();
            let mut array_texture_size = six_textures[0].size;
            array_texture_size.depth = 6;
            let array_texture_format = six_textures[0].format;
            let array_texture = Texture::new(array_texture_size, TextureDimension::D2, array_texture_data, array_texture_format);
            let array_texture_handle = textures.add(array_texture);
            
            println!("Created array_texture!!");
            commands.insert(entity, MeshBundle{
                mesh: skybox_state.cube.clone(),
                render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                    skybox_state.pipeline.clone(),
                )]),
                ..Default::default()
            });
            commands.insert_one(entity, RenderableSkybox{ cubemap_texture: array_texture_handle });
        }
    }
}

pub fn setup_skybox(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut pipeline_descriptor = PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    });
    pipeline_descriptor.name = Some("ironslay_skybox".to_owned());
    pipeline_descriptor.rasterization_state.as_mut().unwrap().cull_mode = CullMode::None;
    pipeline_descriptor.depth_stencil_state.as_mut().unwrap().depth_write_enabled = false;
    let pipeline_handle = pipelines.add(pipeline_descriptor);

    render_graph.add_system_node(
        "ironslay_skybox",
        RenderResourcesNode::<RenderableSkybox>::new(true),
    );

    // Add a Render Graph edge connecting our new "my_material" node to the main pass node. This ensures "my_material" runs before the main pass
    render_graph
        .add_node_edge("ironslay_skybox", base::node::MAIN_PASS)
        .unwrap();

    commands.insert_resource(SkyboxPluginState{
        cube: meshes.add(Mesh::from(shape::Cube::new(1.0))),
        pipeline: pipeline_handle,
    });
}