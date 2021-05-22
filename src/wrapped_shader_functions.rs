// This file is just a helper to convert from and to the different math library types when calling the hex_shader defined functions
use hex_shader;
use bevy;

pub type BevyVec2 = bevy::math::Vec2;
pub type ShaderVec2 = hex_shader::Vec2;

pub mod convert {
    use super::{BevyVec2, ShaderVec2};

    pub fn bevy_to_shader_vec2(vec: BevyVec2) -> ShaderVec2 {
        ShaderVec2::new(vec.x, vec.y)
    }

    pub fn shader_to_bevy_vec2(vec: ShaderVec2) -> BevyVec2 {
        BevyVec2::new(vec.x, vec.y)
    }
}

pub fn hex_dist(p: BevyVec2) -> f32
{
    hex_shader::hex_dist(convert::bevy_to_shader_vec2(p))
}

pub fn hex_relative_uv(uv: BevyVec2) -> BevyVec2
{

    let shader_gv = hex_shader::hex_relative_uv(convert::bevy_to_shader_vec2(uv));
    convert::shader_to_bevy_vec2(shader_gv)
}