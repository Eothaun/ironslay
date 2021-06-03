// Internal
mod components;
mod gameplay_systems;
mod rendering;
mod gameplay_helpers;
mod orbit_camera;
mod wrapped_shader_functions;
mod math_helpers;
use orbit_camera::*;
use components::*;
use rendering::components::*;

// External
use bevy::prelude::*;
use bevy::render::{
    mesh::shape,
    pipeline::{PipelineDescriptor, RenderPipeline},
    render_graph::{RenderGraph}
};
use bevy::ecs::schedule::ReportExecutionOrderAmbiguities;
use bevy::math::IVec2;
use bevy_mod_raycast::{DefaultRaycastingPlugin, RayCastMesh, RaycastSystem};
use bevy_skybox::{SkyboxCamera, SkyboxPlugin};

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
        .insert_resource(ReportExecutionOrderAmbiguities)
        .add_system_to_stage(
            CoreStage::PostUpdate,
            gameplay_systems::update_raycast_with_cursor.system().before(RaycastSystem::BuildRays),
        )
        .add_asset::<MyMaterial>()
        .insert_resource(IronSlayGlobalResources::default())
        .add_startup_system(rendering::systems::setup.system().before("main_init"))
        .add_startup_system(setup.system().label("main_init"))
        .add_system(gameplay_systems::update_hex_selection.system())
        .add_system(gameplay_helpers::update_grid_ids.system())
        //.add_system(gameplay_helpers::debug_print_grid.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ironslay_resources: Res<IronSlayGlobalResources>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut my_materials: ResMut<Assets<MyMaterial>>,
) {
    // load a texture and retrieve its aspect ratio
    let texture_handle = asset_server.load("branding/bevy_logo_dark_big.png");
    let background_handle = asset_server.load("textures/paper_tileable.jpg");
    let aspect = 0.25;

    // Load hexagon cap model
    let hexagon_cap = asset_server.load("models/HexagonCap.gltf#Mesh0/Primitive0");

    // create a new quad mesh. this is what we will apply the texture to
    let quad_width = 8.0;
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        quad_width,
        quad_width * aspect,
    ))));

    // this material renders the texture normally
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        unlit: true,
        ..Default::default()
    });

    // this material modulates the texture to make it red (and slightly transparent)
    let red_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgba(1.0, 0.0, 0.0, 0.5),
        base_color_texture: Some(texture_handle.clone()),
        unlit: true,
        ..Default::default()
    });

    // and lets make this one blue! (and also slightly transparent)
    let blue_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgba(0.0, 0.0, 1.0, 0.5),
        base_color_texture: Some(texture_handle),
        unlit: true,
        ..Default::default()
    });

    // Create a new custom material
    let my_material = my_materials.add(MyMaterial{ 
        color: Color::WHITE, 
        highlighted_id: Vec2::new(5.0, 5.0),
        selected_id: Vec2::new(10.0, 10.0), 
        background_texture: background_handle,   
    });

    commands.insert_resource(HexGrid::new(8, 8));

    // add entities to the world
    // textured quad - normal
    commands.spawn_bundle(PbrBundle {
            mesh: quad_handle.clone(),
            material: material_handle,
            transform: Transform {
                translation: Vec3::new(-8.0, 0.0, 1.5),
                rotation: Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
                ..Default::default()
            },
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            ..Default::default()
        });
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
        // textured quad - modulated
    commands.spawn_bundle(PbrBundle {
            mesh: quad_handle,
            material: blue_material_handle,
            transform: Transform {
                translation: Vec3::new(-8.0, 0.0, -1.5),
                rotation: Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
                ..Default::default()
            },
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            ..Default::default()
        });
        // plane with custom shader
    commands.spawn_bundle(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 2.0 })),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                ironslay_resources.hex_render_pipeline.clone(),
            )]),
            transform: Transform::from_translation(Vec3::new(5.0, -1.0, 0.0)),
            ..Default::default()
        })
        .insert(my_material.clone())
        .insert(RayCastMesh::<HexRaycastLayer>::default());
        // custom mesh
    commands.spawn_bundle(MeshBundle {
            mesh: hexagon_cap,
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                ironslay_resources.hex_render_pipeline.clone(),
            )]),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(my_material)
        .insert(HexRaycastTarget::default())
        .with_children(|parent| {
            for y in 0..8 {
                for x in 0..8 {
                    parent.spawn().insert(GridPosition{position: IVec2::new(x, y)});
                }
            }
        });
        
        // light
    commands.spawn_bundle(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
        // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 8.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..Default::default()
        })
        .insert(OrbitCamera::default())
        .insert(SkyboxCamera)
        .insert(HexRaycastSource::new());
}
