#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr),
    register_attr(spirv)
)]
// HACK(eddyb) can't easily see warnings otherwise from `spirv-builder` builds.
// Uncomment the line below to always see warnings in the console
// #![deny(warnings)]

#![allow(unused_imports)]

#[cfg(not(target_arch = "spirv"))]
#[macro_use]
pub extern crate spirv_std_macros;

use core::f32;

use spirv_std::glam::{ vec2, vec3, vec4 };
use spirv_std::storage_class::{ Input, Output, Uniform };
use spirv_std::num_traits::Float;
use spirv_std::{ Sampler, Image2d };

pub use spirv_std::glam::{Vec2, Vec3, Vec4};

#[spirv(block)]
#[repr(C)]
pub struct MyMaterial_color
{
    color: Vec4
}
#[spirv(block)]
#[repr(C)]
pub struct MyMaterial_highlighted_id
{
    highlighted_id: Vec2,
    // Shader compiler shares struct definitions with the same internal types, so we have to add dummy fields to let the types differ...
    _dummy: f32,
}

#[spirv(block)]
#[repr(C)]
pub struct MyMaterial_selected_id
{
    selected_id: Vec2,
    // Shader compiler shares struct definitions with the same internal types, so we have to add dummy fields to let the types differ...
    _dummy: Vec2,
}

pub fn saturate(x: f32) -> f32 {
    x.max(0.0).min(1.0)
}

pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    // Scale, bias and saturate x to 0..1 range
    let x = saturate((x - edge0) / (edge1 - edge0));
    // Evaluate polynomial
    x * x * (3.0 - 2.0 * x)
}

pub fn step(edge: f32, x: f32) -> f32 {
    if x < edge {
        0.0
    } else {
        1.0
    }
}

pub fn lerp(x: f32, min: f32, max: f32) -> f32 {
    min + (max - min) * x
}

pub fn lerp_clamped(x: f32, min: f32, max: f32) -> f32 {
    let x_clamped = x.clamp(min, max);
    min + (max - min) * x_clamped
}

pub fn vec2_mod(a: Vec2, b: Vec2) -> Vec2
{
    vec2(a.x % b.x, a.y % b.y)
}

pub fn hex_dist(mut p: Vec2) -> f32
{
    p = p.abs();

    let mut c = p.dot(vec2(1.0, 1.73).normalize());
    c = c.max(p.x);

    return c;
}

pub fn hex_relative_uv(uv: Vec2) -> Vec2
{
    let r = vec2(1.0, 1.73);
    let h = r * 0.5;

    let a = vec2_mod(uv, r) - h;
    let b = vec2_mod(uv+h, r) - h;

    if a.length() < b.length() {
        a
    } else {
        b
    }
}


fn selection_color() -> Vec3 { Vec3::new(1.0, 1.0, 0.0) }

#[spirv(fragment)]
pub fn main(
    uv_input: Input<Vec2>,
    
    #[spirv(descriptor_set = 2, binding = 0)] color_uniform: Uniform<MyMaterial_color>,
    #[spirv(descriptor_set = 2, binding = 1)] highlight_uniform: Uniform<MyMaterial_highlighted_id>,
    #[spirv(descriptor_set = 2, binding = 2)] selection_uniform: Uniform<MyMaterial_selected_id>,
    #[spirv(descriptor_set = 2, binding = 3)] MyMaterial_background_texture: Uniform<Image2d>,
    #[spirv(descriptor_set = 2, binding = 4)] MyMaterial_background_texture_sampler: Uniform<Sampler>,

    mut colour_output: Output<Vec4>
) {
    let mut uv: Vec2 = *uv_input;
    uv *= 5.0;

    let gv = hex_relative_uv(uv);
    let hex_dist = 0.5 - hex_dist(gv);
    let id = uv - gv;

    let mut col = Vec3::one();
  
    let target_id_in_fragment = Vec2::from(highlight_uniform.highlighted_id).distance_squared(id) < 0.1;
    col *= Vec3::splat(lerp(target_id_in_fragment as i32 as f32, 0.4, 1.0));

    let selected_id_in_fragment = Vec2::from(selection_uniform.selected_id).distance_squared(id) < 0.1;
    col *= Vec3::one().lerp(selection_color(), selected_id_in_fragment as i32 as f32);

    let fragment_in_border = hex_dist < 0.04;
    col += Vec3::splat(fragment_in_border as i32 as f32);

    let diffuse_texture_color = MyMaterial_background_texture.sample(*MyMaterial_background_texture_sampler, *uv_input);
    *colour_output = diffuse_texture_color * col.extend(1.0) * color_uniform.color;
}