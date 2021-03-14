#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr),
    register_attr(spirv)
)]
// HACK(eddyb) can't easily see warnings otherwise from `spirv-builder` builds.
#![deny(warnings)]

#[cfg(not(target_arch = "spirv"))]
#[macro_use]
pub extern crate spirv_std_macros;
use spirv_std::glam::{
    // vec4, 
    Vec4, 
    // Vec3,
    // Mat4
};
use spirv_std::storage_class::{
    // Input, 
    Output,
    // Uniform
};

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

#[spirv(fragment)]
pub fn main(
    mut output: Output<Vec4>
) {
    *output = Vec4::new(1.0, 0.0, 0.0, 1.0);
}