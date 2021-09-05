#version 450

// Attributes
// ============================================================================
// Input
layout(location = 0) in vec2 i_Uv;
// Output 
layout(location = 0) out vec4 o_Target;

// Uniforms
layout(set = 2, binding = 0) uniform HexMaterial_color {
    vec4 color;
};
layout(set = 2, binding = 1) uniform HexMaterial_highlighted_id {
    vec2 highlighted_id;
};
layout(set = 2, binding = 2) uniform HexMaterial_selected_id {
    vec2 selected_id;
};
layout(set = 2, binding = 3) uniform texture2D HexMaterial_background_texture;
layout(set = 2, binding = 4) uniform sampler HexMaterial_background_texture_sampler;
layout(set = 2, binding = 5) uniform utexture2D HexMaterial_map_state;
layout(set = 2, binding = 6) uniform sampler HexMaterial_map_state_sampler;
// ============================================================================


// Hex functions
float hex_dist(vec2 p) {
    p = abs(p);

    float c = dot(p, normalize(vec2(1.0, 1.73)));
    c = max(c, p.x);

    return c;
}

vec2 hex_relative_uv(vec2 uv) {
    vec2 r = vec2(1.0, 1.73);
    vec2 h = r * 0.5;

    vec2 a = mod(uv, r) - h;
    vec2 b = mod(uv + h, r) - h;

    if (length(a) < length(b)) {
        return a;
    } else {
        return b;
    }
}


// Fragment shader
void main() {
    vec2 uv = i_Uv * 5.0;

    vec2 gv = hex_relative_uv(uv);
    float hex_dist = 0.5 - hex_dist(gv);
    vec2 id = uv - gv;

    vec3 col = color.rgb;

    bool target_id_in_fragment = distance(id, highlighted_id) < 0.1;
    col *= mix(0.4, 1.0, float(target_id_in_fragment));

    bool selected_id_in_fragment = distance(id, selected_id) < 0.1;
    col *= mix(vec3(1.0), vec3(1.0, 1.0, 0.2), float(selected_id_in_fragment));

    bool fragment_in_border = hex_dist < 0.04;
    col += vec3(float(fragment_in_border));

    col *= texture(sampler2D(HexMaterial_background_texture, HexMaterial_background_texture_sampler), i_Uv).xyz;

    // TODO FIX: I think the `id` variable doesn't line up with the pixel coords here
    uint map_data = texelFetch(usampler2D(HexMaterial_map_state, HexMaterial_map_state_sampler), ivec2(id), 0).r;
    if(map_data == 0)
        col *= vec3(0.0, 1.0, 0.0);
    else if(map_data == 1)
        col *= vec3(0.0, 0.0, 1.0);

    o_Target = vec4(col.rgb, color.a);
}
