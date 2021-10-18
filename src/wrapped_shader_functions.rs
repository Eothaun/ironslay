// This file is just a helper to convert from and to the different math library types when calling the hex_shader defined functions
//use hex_shader;
use bevy;

pub type BevyVec2 = bevy::math::Vec2;
pub type BevyIVec2 = bevy::math::IVec2;
// pub type ShaderVec2 = hex_shader::Vec2;

// pub mod convert {
//     use super::{BevyVec2, ShaderVec2};

//     pub fn bevy_to_shader_vec2(vec: BevyVec2) -> ShaderVec2 {
//         ShaderVec2::new(vec.x, vec.y)
//     }

//     pub fn shader_to_bevy_vec2(vec: ShaderVec2) -> BevyVec2 {
//         BevyVec2::new(vec.x, vec.y)
//     }
// }

mod hex_shader_copy_pasted {
    use bevy::math::prelude::*;
    use crate::math_helpers::vec2_mod;

    pub fn hex_dist(mut p: Vec2) -> f32 {
        p = p.abs();
    
        let mut c = p.dot(Vec2::new(1.0, 1.73).normalize());
        c = c.max(p.x);
    
        return c;
    }
    
    pub fn hex_relative_uv(uv: Vec2) -> Vec2 {
        let r = Vec2::new(1.0, 1.73);
        let h = r * 0.5;
    
        let a = vec2_mod(uv, r) - h;
        let b = vec2_mod(uv + h, r) - h;
    
        if a.length() < b.length() {
            a
        } else {
            b
        }
    }

    pub fn hex_grid_coord(id: Vec2) -> Vec2 {
        let r = Vec2::new(1.0, 1.73);
        let h = r * 0.5;
        (id / h)//.as_i32()
    }
}

pub fn hex_dist(p: BevyVec2) -> f32 {
    //hex_shader::hex_dist(convert::bevy_to_shader_vec2(p))
    hex_shader_copy_pasted::hex_dist(p)
}

pub fn hex_relative_uv(uv: BevyVec2) -> BevyVec2 {
    //let shader_gv = hex_shader::hex_relative_uv(convert::bevy_to_shader_vec2(uv));
    //convert::shader_to_bevy_vec2(shader_gv)
    hex_shader_copy_pasted::hex_relative_uv(uv)
}

pub fn hex_grid_coord(id: BevyVec2) -> BevyVec2 {
    hex_shader_copy_pasted::hex_grid_coord(id)
}