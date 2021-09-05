// Internal
mod components;
mod gameplay_helpers;
mod gameplay_systems;
mod math_helpers;
mod orbit_camera;
mod rendering;
mod wrapped_shader_functions;
use components::*;
use orbit_camera::*;
use rendering::components::*;
mod ui;

// External
use bevy::ecs::schedule::ReportExecutionOrderAmbiguities;
use bevy::math::IVec2;
use bevy::prelude::*;
use bevy::render::{
    mesh::shape,
    pipeline::{PipelineDescriptor, RenderPipeline},
    texture::{Extent3d, TextureFormat, TextureDimension, FilterMode},
};
use bevy_mod_raycast::{DefaultRaycastingPlugin, RayCastMesh, RaycastSystem};
use bevy_skybox::{SkyboxCamera, SkyboxPlugin};

use bytemuck;

use std::env;

#[derive(Clone, Default)]

pub struct IronSlayGlobalResources {
    hex_render_pipeline: Handle<PipelineDescriptor>,
}

fn main() {
    let path = env::current_dir().unwrap();
    println!("The current working directory is {}", path.display());

    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(OrbitCameraPlugin)
        .add_plugin(DefaultRaycastingPlugin::<HexRaycastLayer>::default())
        .add_plugin(SkyboxPlugin::from_image_file("sky1.png"))
        // add this resource to your App to enable ambiguity detection
        //.insert_resource(ReportExecutionOrderAmbiguities)
        .add_system_to_stage(
            CoreStage::PostUpdate,
            gameplay_systems::update_raycast_with_cursor.system().before(RaycastSystem::BuildRays),
        )
        .add_asset::<HexMaterial>()
        .insert_resource(IronSlayGlobalResources::default())
        .add_startup_system(rendering::systems::setup.system().before("main_init"))
        .add_startup_system(setup.system().label("main_init"))
        .add_system(gameplay_systems::update_mouse_hovering_and_selected.system())
        .add_system(gameplay_helpers::update_grid_ids.system())
        .add_system_to_stage(CoreStage::PostUpdate, rendering::systems::update_map_texture.system())
        .add_state(GameState::default())
        //.add_system(gameplay_helpers::debug_print_grid.system())
        .add_plugin(ui::UIPlugins)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ironslay_resources: Res<IronSlayGlobalResources>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    mut hex_materials: ResMut<Assets<HexMaterial>>,
) {
    // load a texture and retrieve its aspect ratio
    let texture_handle = asset_server.load("branding/bevy_logo_dark_big.png");
    let background_handle = asset_server.load("textures/paper_tileable.jpg");
    
    let map_data_vec: Vec<u32> = vec![0; 8*8];
    let mut map_texture = Texture::new_fill(Extent3d::new(8, 8, 1), TextureDimension::D2, 
    bytemuck::cast_slice(map_data_vec.as_slice()), TextureFormat::R32Uint);
    map_texture.sampler.min_filter = FilterMode::Nearest;
    map_texture.sampler.mag_filter = FilterMode::Nearest;

    // Load hexagon cap model
    let hexagon_cap = asset_server.load("models/HexagonCap.gltf#Mesh0/Primitive0");

    // create a new quad mesh. this is what we will apply the texture to
    let quad_width = 8.0;
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        quad_width,
        quad_width * 0.25,
    ))));

    // this material modulates the texture to make it red (and slightly transparent)
    let red_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgba(1.0, 0.0, 0.0, 0.5),
        base_color_texture: Some(texture_handle.clone()),
        unlit: true,
        ..Default::default()
    });

    // Create a new custom material
    let hex_material = hex_materials.add(HexMaterial {
        color: Color::WHITE,
        highlighted_id: Vec2::new(5.0, 5.0),
        selected_id: Vec2::new(10.0, 10.0),
        background_texture: background_handle,
        map_state: textures.add(map_texture)
    });

    commands.insert_resource(HexGrid::new(8, 8));

    // add entities to the world
    // textured quad - modulated
    commands.spawn_bundle(PbrBundle {
        mesh: quad_handle.clone(),
        material: red_material_handle,
        transform: Transform {
            translation: Vec3::new(-8.0, 0.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
            ..Default::default()
        },
        visible: Visible {
            is_transparent: true,
            ..Default::default()
        },
        ..Default::default()
    });
    // plane with hex shader
    commands
        .spawn_bundle(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 2.0 })),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                ironslay_resources.hex_render_pipeline.clone(),
            )]),
            transform: Transform::from_translation(Vec3::new(5.0, -1.0, 0.0)),
            ..Default::default()
        })
        .insert(hex_material.clone())
        .insert(RayCastMesh::<HexRaycastLayer>::default());
    // planet mesh with hex shader
    commands
        .spawn_bundle(MeshBundle {
            mesh: hexagon_cap,
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                ironslay_resources.hex_render_pipeline.clone(),
            )]),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(hex_material)
        .insert(HexRaycastTarget::default())
        // Hex spawning...
        .with_children(|parent| {
            for y in 0..8 {
                for x in 0..8 {
                    parent.spawn()
                        .insert(GridPosition { position: IVec2::new(x, y), })
                        .insert(if x < 3 || y < 3 { TerrainType::Water } else { TerrainType::Land });
                }
            }
        });
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 8.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..Default::default()
        })
        .insert(OrbitCamera::default())
        .insert(SkyboxCamera)
        .insert(HexRaycastSource::new());
}
