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

use spirv_std::glam::{
    Vec2, 
    Vec3,
    Vec4, 
    vec2,
    vec3,
    vec4,
};
use spirv_std::storage_class::{
    Input, 
    Output
};
use spirv_std::num_traits::Float;

// This is kept here for future reference
// #[spirv(block)]
// pub struct Camera {
// 	pub view_proj: Mat4
// }

// #[spirv(block)]
// pub struct Transform {
// 	pub model: Mat4
// }

// #[spirv(vertex)]
// pub fn main_vs(
//     in_pos: Input<Vec3>,
//     #[spirv(descriptor_set = 0, binding = 0)] camera: Uniform<Camera>,
//     #[spirv(descriptor_set = 1, binding = 0)] transform: Uniform<Transform>,
//     #[spirv(position)] mut out_pos: Output<Vec4>,
// ) {
//     *out_pos = camera.view_proj * transform.model * vec4(
//         in_pos.x,
//         in_pos.y,
//         in_pos.z,
//         1.0,
//     );
// }

pub fn saturate(x: f32) -> f32 {
    x.max(0.0).min(1.0)
}

pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    // Scale, bias and saturate x to 0..1 range
    let x = saturate((x - edge0) / (edge1 - edge0));
    // Evaluate polynomial
    x * x * (3.0 - 2.0 * x)
}

fn step(edge: f32, x: f32) -> f32
{
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

    let gv = if a.length() < b.length() {
        a
    } else {
        b
    };

    gv
}

#[spirv(fragment)]
pub fn main(
    uv_input: Input<Vec2>,
    mut colour_output: Output<Vec4>
) {
    let mut uv: Vec2 = *uv_input;
    uv *= 5.0;
    uv += Vec2::splat(2.0);

    let mut col = vec3(0.0, 0.0, 0.0);

    let gv = hex_relative_uv(uv);
    let hex_dist = 0.5 - hex_dist(gv);
    let id = uv - gv;
    col += Vec3::splat(smoothstep(0.0, 0.1, hex_dist));

    *colour_output = col.extend(1.0);
}