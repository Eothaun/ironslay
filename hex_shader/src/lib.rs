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

use spirv_std::glam::{ Vec2, Vec3, Vec4, vec2, vec3, vec4 };
use spirv_std::storage_class::{ Input, Output, Uniform };
use spirv_std::num_traits::Float;


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
    highlighted_id: Vec2
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

fn step(edge: f32, x: f32) -> f32 {
    if x < edge {
        0.0
    } else {
        1.0
    }
}

fn vec2_modulo(a: Vec2, b: Vec2) -> Vec2
{
    vec2(a.x % b.x, a.y % b.y)
}

fn hex_dist(mut p: Vec2) -> f32
{
    p = p.abs();

    let mut c = p.dot(vec2(1.0, 1.73).normalize());
    c = c.max(p.x);

    return c;
}

fn hex_relative_uv(uv: Vec2) -> Vec2
{
    let r = vec2(1.0, 1.73);
    let h = r * 0.5;

    let a = vec2_modulo(uv, r) - h;
    let b = vec2_modulo(uv+h, r) - h;

    if a.length() < b.length() {
        a
    } else {
        b
    }
}

#[spirv(fragment)]
pub fn main(
    uv_input: Input<Vec2>,
    
    #[spirv(descriptor_set = 2, binding = 0)] color_uniform: Uniform<MyMaterial_color>,
    #[spirv(descriptor_set = 2, binding = 1)] highlight_uniform: Uniform<MyMaterial_highlighted_id>,

    mut colour_output: Output<Vec4>
) {
    let mut uv: Vec2 = *uv_input;
    uv *= 5.0;
    uv += Vec2::splat(2.0);

    let mut col = vec3(1.0, 1.0, 1.0);

    let mut target_uv = highlight_uniform.highlighted_id;
    target_uv *= 5.0;
    target_uv += Vec2::splat(2.0);
    let uv_dist = (1.0-(target_uv - uv).length()).max(0.1);
    col *= Vec3::splat(uv_dist);

    let gv = hex_relative_uv(uv);
    let hex_dist = 0.5 - hex_dist(gv);
    let id = uv - gv;
    col *= Vec3::splat(smoothstep(0.0, 0.1, hex_dist));

    *colour_output = col.extend(1.0) * color_uniform.color;
}