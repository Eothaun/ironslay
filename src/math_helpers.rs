use bevy::math::{Vec2, Vec3};


pub fn vec3_all_eq(a: Vec3, b: Vec3, epsilon: f32) -> bool {
    (a.x - b.x).abs() <= epsilon && (a.y - b.y).abs() <= epsilon && (a.z - b.z).abs() <= epsilon
}

pub fn vec2_mod(a: Vec2, b: Vec2) -> Vec2 {
    Vec2::new(a.x % b.x, a.y % b.y)
}

// From https://gamedev.stackexchange.com/questions/23743/whats-the-most-efficient-way-to-find-barycentric-coordinates
pub fn calculate_barycentric_coords(vertex_a: Vec3, vertex_b: Vec3, vertex_c: Vec3, pos: Vec3) -> Vec3 {
    let v0: Vec3 = vertex_b - vertex_a;
    let v1: Vec3 = vertex_c - vertex_a;
    let v2: Vec3 = pos - vertex_a;
    let d00 = v0.dot(v0);
    let d01 = v0.dot(v1);
    let d11 = v1.dot(v1);
    let d20 = v2.dot(v0);
    let d21 = v2.dot(v1);
    let denom = d00 * d11 - d01 * d01;

    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;

    // Sanity test
    let reconstructed_pos: Vec3 = vertex_a * u + vertex_b * v + vertex_c * w;
    assert!(vec3_all_eq(reconstructed_pos, pos, 0.01));

    Vec3::new(u, v, w)
}